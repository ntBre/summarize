use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader},
    iter::zip,
    path::Path,
    str::FromStr,
};

use coriolis::Coriol;
use curvil::Curvil;
use delta::Delta;
use lazy_static::lazy_static;
use phi::Phi;
use regex::Regex;
use serde::Serialize;
use symm::{Atom, Irrep, Molecule};

#[cfg(test)]
mod tests;

macro_rules! write_dist_consts {
    ($w:ident, $struct:ident, $($field:ident => $name:expr$(,)?),*) => {
	$(
	    if let Some(d) = $struct.$field {
		write!($w, "{:<13}{:18.10}", $name, d)?;
	    }
	)*
    };
}

pub mod delta;
pub mod phi;

pub mod curvil {
    use serde::Serialize;

    /// represented only by the indices into the geometry
    #[derive(Debug, PartialEq, Serialize)]
    pub enum Curvil {
        Bond(usize, usize),
        Angle(usize, usize, usize),
        Torsion(usize, usize, usize, usize),

        /// a linear bend
        Linear(usize, usize, usize),
    }
}

const BAD_FLOAT: f64 = 999999999.9;

/// threshold for discarding rotations and translations
const ROTRANS_THRSH: f64 = 30.0;

/// threshold for computing irrep symmetries
const SYMM_EPS: f64 = 1e-4;

pub const TO_MHZ: f64 = 29979.2458;

/// for use with map
pub fn to_mhz(v: &f64) -> f64 {
    v * TO_MHZ
}

const DEBUG: bool = false;

lazy_static! {
    /// default weights used in SPECTRO
    static ref ATOMIC_WEIGHTS: HashMap<&'static str, usize> = HashMap::from([
        ("1.0078250", 1),
        ("4.0026032", 2),
        ("7.0160030", 3),
        ("9.0121822", 4),
        ("11.0093054", 5),
        ("12.0000000", 6),
        ("14.0030740", 7),
        ("15.9949146", 8),
        ("18.9984032", 9),
        ("19.9924356", 10),
        ("22.9897677", 11),
        ("23.9850423", 12),
        ("26.9815386", 13),
        ("27.9769271", 14),
        ("30.9737620", 15),
        ("31.9720707", 16),
        ("34.9688527", 17),
        ("39.9623837", 18),
    ]);
    static ref HEADER: Regex = Regex::new(r"^(\s*\d+)+\s*$").unwrap();
    static ref DISP: Regex = Regex::new(r"^\d+$").unwrap();
    static ref DELTA: Regex = Regex::new(r"(?i)(^  d(elta)? [jk12]+ |^ De\b)").unwrap();
    static ref PHI: Regex = Regex::new(r"(?i)(^  (p)?h(i)? [jk123]+ |^ He\b)").unwrap();
    static ref FERMI: Regex = Regex::new(r"(?i)^ INPUTED FERMI").unwrap();
    static ref CORIOL: Regex = Regex::new(r"(?i)^ INPUTED CORIOLIS").unwrap();
    /// empty line
    static ref BLANK: Regex = Regex::new(r"^\s*$").unwrap();
    /// can also occur at the end of the CURVIL section
    static ref OPTDL: Regex = Regex::new(r"^ I OPTDL").unwrap();
    static ref COORD: Regex = Regex::new(r"VIBRATIONALLY AVERAGED COORDINATES").unwrap();
    static ref CURVIL: Regex = Regex::new(r"^ CURVILINEAR INTERNAL COORDINATES").unwrap();
}

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct Summary {
    /// harmonic vibrational frequencies
    pub harm: Vec<f64>,

    /// partially resonance-corrected anharmonic vibrational frequencies
    pub fund: Vec<f64>,

    /// fully resonance-corrected anharmonic vibrational frequencies
    pub corr: Vec<f64>,

    /// molecular geometry used in the other calculations
    pub geom: Molecule,

    /// symmetries of the vibrational modes, probably not the best name
    pub irreps: Vec<Irrep>,

    /// normal coordinate matrix corresponding to the harmonic vibrational
    /// frequencies in `harm`
    pub lxm: Vec<Vec<f64>>,

    /// vibrationally-averaged (₀) and singly-vibrationally-excited rotational
    /// constants
    pub rots: Vec<Vec<f64>>,

    /// equilibrium (ₑ) rotational constants
    pub rot_equil: Vec<f64>,

    /// quartic distortion coefficients
    pub deltas: Delta,

    /// sextic distortion coefficients
    pub phis: Phi,

    /// inputed type-1 and -2 fermi resonances. map of each mode to its
    /// equivalences. Note that these are not necessarily the resonances
    /// detected *or* used by spectro, only the ones reported in the "INPUTED
    /// ... RESONANCE DATA" section
    pub fermi: HashMap<usize, Vec<(usize, usize)>>,

    /// inputed coriolis resonances. map of modes to axes
    pub coriolis: coriolis::Coriol,

    /// zero-point vibrational energy
    pub zpt: f64,

    /// inputted curvilinear internal coordinates
    pub curvils: Vec<Curvil>,

    /// R(ALPHA) values of the curvilinear coordinates in `curvil`
    pub ralpha: Vec<f64>,

    /// R(EQUIL) values of the curvilinear coordinates in `curvil`
    pub requil: Vec<f64>,
}

mod coriolis {
    use std::collections::HashMap;

    use serde::{ser::SerializeStruct, Serialize};

    #[derive(Default, Debug, PartialEq)]
    pub struct Coriol {
        pub data: HashMap<(usize, usize), Vec<usize>>,
    }

    impl Serialize for Coriol {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut s = serializer.serialize_struct("Coriol", 2)?;
            let mut modes = Vec::new();
            let mut axes = Vec::new();
            let mut keys: Vec<_> = self.data.keys().collect();
            keys.sort();
            for key in keys {
                modes.push(key);
                axes.push(self.data[key].clone());
            }
            s.serialize_field("modes", &modes)?;
            s.serialize_field("axes", &axes)?;
            s.end()
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Fund,
    Corr,
    Geom,
    Lxm,
    RotA,
    RotS,
    Fermi1,
    Fermi2,
    Coriolis,
    Curvil,
    Coords,
    None,
}

impl State {
    fn is_fermi(&self) -> bool {
        matches!(self, State::Fermi1 | State::Fermi2)
    }

    fn is_coords(&self) -> bool {
        matches!(self, State::Coords)
    }

    /// Returns `true` if the state is [`Curvil`].
    ///
    /// [`Curvil`]: State::Curvil
    #[must_use]
    fn is_curvil(&self) -> bool {
        matches!(self, Self::Curvil)
    }
}

impl Summary {
    pub fn new<P>(filename: P) -> Self
    where
        P: AsRef<Path> + std::fmt::Display,
    {
        let f = match std::fs::File::open(&filename) {
            Ok(f) => f,
            Err(e) => panic!("failed to open {} with '{}'", filename, e),
        };
        let ext = filename.as_ref().extension().unwrap_or_default();
        if ext == "json" {
            let f = std::fs::File::open(&filename).unwrap();
            let output: spectro::Output = serde_json::from_reader(f).unwrap();
            return Summary::from(output);
        }
        let lines = BufReader::new(f).lines().flatten();

        let mut state = State::None;
        let mut skip = 0;
        let mut ret = Self::default();
        // these are for multi-line state descriptions
        let mut vib_states = Vec::new();
        let mut cur_zpt = 0.0;
        let mut cur_freq = 0.0;
        // keep track of the freqs from the LXM matrix to handle degenerate
        // modes
        let mut lxm_freqs = Vec::new();
        // block of the LXM matrix
        let mut block = 0;
        // rotational constant variables
        let mut rot_good = false;
        let mut rot_states = Vec::new();
        'outer: for line in lines {
            if skip > 0 {
                skip -= 1;
            } else if line.contains("MOLECULAR PRINCIPAL GEOMETRY") {
                skip = 2;
                state = State::Geom;
            } else if state == State::Geom {
                geom_handler(&line, &mut state, &mut ret);
            } else if line.contains("LXM MATRIX") {
                skip = 2;
                state = State::Lxm;
                // reset these. for degmodes it gets printed twice
                block = 0;
                lxm_freqs = Vec::new();
                ret.lxm = Vec::new();
            } else if state == State::Lxm {
                let fields: Vec<_> = line.split_whitespace().collect();
                if fields.is_empty() {
                    skip = 1;
                } else if HEADER.is_match(&line) {
                    block += 1;
                    continue;
                } else if line.contains("--------") {
                    continue;
                } else if line.contains("LX MATRIX") {
                    state = State::None;
                } else if DISP.is_match(fields[0]) {
                    for (i, d) in fields[1..].iter().enumerate() {
                        let idx = 10 * block + i;
                        if ret.lxm.len() <= idx {
                            ret.lxm.resize(idx + 1, vec![]);
                        }
                        ret.lxm[idx]
                            .push(f64::from_str(d).unwrap_or(BAD_FLOAT));
                    }
                } else {
                    lxm_freqs.extend(fields.iter().filter_map(|s| {
                        let f = s.parse::<f64>().unwrap_or(BAD_FLOAT);
                        if f > ROTRANS_THRSH {
                            Some(f)
                        } else {
                            None
                        }
                    }));
                }
            } else if line.contains("BAND CENTER ANALYSIS") {
                skip = 3;
                state = State::Fund;
            } else if line.contains("DUNHAM")
                || line.contains("VIBRATIONAL ENERGY AND")
            {
                state = State::None;
            } else if state == State::Fund
                && line.contains(|s: char| s.is_numeric())
            {
                let fields: Vec<_> = line.split_whitespace().collect();
                ret.harm.push(fields[1].parse().unwrap_or(BAD_FLOAT));
                ret.fund.push(fields[2].parse().unwrap_or(BAD_FLOAT));
            } else if line.contains("STATE NO.") && !line.contains("SPECTRUM") {
                skip = 2;
                state = State::Corr;
            } else if state == State::Corr
                && line.contains("*******************")
            {
                state = State::None;
            } else if state == State::Corr && line.contains("NON-DEG (Vs)") {
                vib_states.clear();
                let fields: Vec<_> = line.split_whitespace().collect();
                vib_states.extend(
                    fields[6..].iter().map(|s| s.parse::<usize>().unwrap()),
                );
                cur_zpt = fields[1].parse().unwrap_or(BAD_FLOAT);
                cur_freq = fields[2].parse().unwrap_or(BAD_FLOAT);
            } else if state == State::Corr && line.contains("DEGEN   (Vt)") {
                vib_states.extend(
                    line.split_whitespace()
                        .skip(3)
                        .map(|s| s.parse::<usize>().unwrap()),
                );
            } else if state == State::Corr && line.contains("DEGEN   (Vl)") {
                // nothing for now, just eat the line after handling the count
                // above in the Vt case
            } else if state == State::Corr && line.contains("<>") {
                state = State::None;
            } else if state == State::Corr && !line.is_empty() {
                vib_states.extend(
                    line.split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap()),
                );
            } else if state == State::Corr
                && line.is_empty()
                && !vib_states.is_empty()
            {
                if vib_states.iter().all(|s| *s == 0) {
                    ret.zpt = cur_zpt;
                } else {
                    let mut one = false;
                    let mut idx = 0;
                    for (i, state) in vib_states.iter().enumerate() {
                        if (*state == 1 && one) || *state == 2 {
                            continue 'outer;
                        } else if *state == 1 {
                            idx = i;
                            one = true;
                        }
                    }
                    if idx >= ret.corr.len() {
                        ret.corr.resize(idx + 1, 0.0);
                    }
                    ret.corr[idx] = cur_freq;
                }
            } else if line.contains("NON-DEG(Vt)") {
                rot_states.extend(
                    line.split_whitespace().skip(2).map(|s| s.to_string()),
                );
            } else if line.contains("ROTATIONAL ENERGY LEVEL ANALYSIS") {
                state = State::RotA;
                rot_good = true;
            } else if rot_good && line.contains("BZA") {
                state = State::RotA;
            } else if rot_good
                && ret.deltas.de.is_some()
                && line.contains("BZS")
            {
                state = State::RotS;
            } else if COORD.is_match(&line) {
                state = State::Coords;
                skip = 12 + i32::from(ret.rot_equil.len() == 1);
            } else if state == State::RotA && rot_good {
                let mut one = false;
                for state in &rot_states {
                    if (*state == "1" && one) || *state == "2" {
                        continue 'outer;
                    } else if *state == "1" {
                        one = true;
                    }
                }
                rot_states.clear();
                state = State::None;
                let fields: Vec<_> = line.split_whitespace().collect();
                if fields.len() != 3 {
                    // this says "sad hack" next to it in the Go version, not
                    // sure why yet
                    continue;
                }
                let mut v: Vec<_> = fields
                    .iter()
                    .map(|s| s.parse().unwrap_or(f64::NAN) * TO_MHZ)
                    .collect();
                v.sort_by(|a, b| {
                    b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal)
                });
                ret.rots.push(v);
            } else if state == State::RotS && rot_good {
                let mut one = false;
                for state in &rot_states {
                    if (*state == "1" && one) || *state == "2" {
                        continue 'outer;
                    } else if *state == "1" {
                        one = true;
                    }
                }
                rot_states.clear();
                state = State::None;
                let fields: Vec<_> = line.split_whitespace().collect();
                if fields.len() != 3 {
                    // this says "sad hack" next to it in the Go version, not
                    // sure why yet
                    continue;
                }
                let mut v: Vec<_> = fields
                    .iter()
                    .map(|s| s.parse().unwrap_or(f64::NAN) * TO_MHZ)
                    .collect();
                v.sort_by(|a, b| {
                    b.abs()
                        .partial_cmp(&a.abs())
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                // for linear molecules, there is only one unique rotational
                // constant, reported twice, and the third one is zero. sorting
                // by abs moves the real one to the front. for some reason,
                // spectro also reports it as the difference from equilibrium so
                // add to that.
                ret.rots.push(vec![v[0] + ret.rot_equil[0]]);
            } else if line.contains("Be") {
                // line like  ' (Be =    1.64769 IN CM-1)'
                ret.rot_equil.push(
                    line.split_ascii_whitespace()
                        .nth(2)
                        .unwrap()
                        .parse::<f64>()
                        .unwrap_or(f64::NAN)
                        * TO_MHZ,
                );
            } else if DELTA.is_match(&line) {
                let sp: Vec<&str> = line.split_ascii_whitespace().collect();
                if sp.len() > 3 {
                    let v: f64 = sp[4].parse().unwrap_or(f64::NAN);
                    match (sp[0], sp[1]) {
                        // A reduction
                        ("DELTA", "J") => ret.deltas.big_delta_j = Some(v),
                        ("DELTA", "K") => ret.deltas.big_delta_k = Some(v),
                        ("DELTA", "JK") => ret.deltas.big_delta_jk = Some(v),
                        ("delta", "J") => ret.deltas.delta_j = Some(v),
                        ("delta", "K") => ret.deltas.delta_k = Some(v),
                        // S reduction
                        ("D", "J") => ret.deltas.d_j = Some(v),
                        ("D", "JK") => ret.deltas.d_jk = Some(v),
                        ("D", "K") => ret.deltas.d_k = Some(v),
                        ("d", "1") => ret.deltas.d1 = Some(v),
                        ("d", "2") => ret.deltas.d2 = Some(v),
                        _ => panic!(
                            "failed to match '{}' and '{}'",
                            sp[0], sp[1]
                        ),
                    }
                } else {
                    // linear
                    ret.deltas.de = Some(sp[2].parse().unwrap_or(f64::NAN));
                }
            } else if PHI.is_match(&line) {
                let sp: Vec<&str> = line.split_ascii_whitespace().collect();
                if sp.len() > 3 {
                    // phi is in Hz in the file, so turn it to MHz
                    let v: f64 = sp[4]
                        .replace('D', "E")
                        .parse::<f64>()
                        .unwrap_or(f64::NAN)
                        / 1e6;
                    match (sp[0], sp[1]) {
                        // A reduction
                        ("PHI", "J") => ret.phis.big_phi_j = Some(v),
                        ("PHI", "K") => ret.phis.big_phi_k = Some(v),
                        ("PHI", "JK") => ret.phis.big_phi_jk = Some(v),
                        ("PHI", "KJ") => ret.phis.big_phi_kj = Some(v),
                        ("phi", "j") => ret.phis.phi_j = Some(v),
                        ("phi", "jk") => ret.phis.phi_jk = Some(v),
                        ("phi", "k") => ret.phis.phi_k = Some(v),
                        // S reduction
                        ("H", "J") => ret.phis.h_j = Some(v),
                        ("H", "JK") => ret.phis.h_jk = Some(v),
                        ("H", "KJ") => ret.phis.h_kj = Some(v),
                        ("H", "K") => ret.phis.h_k = Some(v),
                        ("h", "1") => ret.phis.h1 = Some(v),
                        ("h", "2") => ret.phis.h2 = Some(v),
                        ("h", "3") => ret.phis.h3 = Some(v),
                        _ => panic!(
                            "failed to match '{}' and '{}'",
                            sp[0], sp[1]
                        ),
                    }
                } else {
                    // linear molecule
                    ret.phis.he = Some(
                        sp[2]
                            .replace('D', "E")
                            .parse::<f64>()
                            .unwrap_or(f64::NAN)
                            / 1e6,
                    );
                }
            } else if FERMI.is_match(&line) {
                let v = line.split_ascii_whitespace().nth(2).unwrap();
                if v == "1" {
                    state = State::Fermi1;
                    skip = 2;
                } else if v == "2" {
                    state = State::Fermi2;
                    skip = 3;
                }
            } else if state.is_fermi() && BLANK.is_match(&line) {
                state = State::None;
            } else if state.is_fermi() {
                let mut v = line.split_ascii_whitespace();
                let a = v.next().unwrap().parse::<usize>().unwrap();
                if state == State::Fermi1 {
                    let b = v.next().unwrap().parse::<usize>().unwrap();
                    let e = ret.fermi.entry(b).or_default();
                    e.push((a, a));
                } else if state == State::Fermi2 {
                    // skip the + connecting two parts of a Fermi 2
                    let b = v.nth(1).unwrap().parse::<usize>().unwrap();
                    let c = v.next().unwrap().parse::<usize>().unwrap();
                    let e = ret.fermi.entry(c).or_default();
                    e.push((a, b));
                }
            } else if CORIOL.is_match(&line) {
                state = State::Coriolis;
                skip = 2;
            } else if state == State::Coriolis && BLANK.is_match(&line) {
                state = State::None;
            } else if state == State::Coriolis {
                let mut v = line.split_ascii_whitespace();
                let a = v.next().unwrap().parse::<usize>().unwrap();
                let b = v.next().unwrap().parse::<usize>().unwrap();
                let axis = v.next().unwrap().parse::<usize>().unwrap();
                let e = ret.coriolis.data.entry((a, b)).or_default();
                e.push(axis);
            } else if state.is_coords() && BLANK.is_match(&line) {
                state = State::None;
            } else if state.is_coords() {
                let v = line.split_ascii_whitespace().collect::<Vec<_>>();
                // have to skip "ANGLE" for linear angles
                let off = usize::from(v[1] == "LINEAR");
                ret.requil.push(v[2 + off].parse().unwrap());
                ret.ralpha.push(v[4 + off].parse().unwrap());
            } else if CURVIL.is_match(&line) {
                state = State::Curvil;
                skip = 4;
            } else if state.is_curvil() {
                if BLANK.is_match(&line) || OPTDL.is_match(&line) {
                    state = State::None;
                    continue;
                }
                let mut in_parens = false;
                let mut new_line = String::with_capacity(line.len());
                // trying to parse a line like:
                //
                // ( 1)        BOND               1.32539       2(C )     3(C )
                //
                // where the atom labels can be two atoms wide, so we can't just
                // split on whitespace
                for c in line.chars() {
                    if c == '(' {
                        new_line.push(' ');
                        in_parens = true;
                    } else if c == ')' {
                        new_line.push(' ');
                        in_parens = false;
                    } else if in_parens && c == ' ' {
                        continue;
                    } else {
                        new_line.push(c);
                    }
                }
                let mut v = new_line.split_ascii_whitespace();
                let typ = v.nth(1).unwrap();
                let ids: Vec<_> =
                    v.skip(1).step_by(2).flat_map(usize::from_str).collect();
                use curvil::Curvil::*;
                ret.curvils.push(match typ {
                    "BOND" => Bond(ids[0], ids[1]),
                    "ANGLE" => Angle(ids[0], ids[1], ids[2]),
                    "TORSION" => Torsion(ids[0], ids[1], ids[2], ids[3]),
                    "LINEAR" => {
                        // the label is actually "LINEAR ANGLE" so we have to
                        // re-process the line
                        let v = new_line.split_ascii_whitespace();
                        let ids: Vec<_> = v
                            .skip(2)
                            .step_by(2)
                            .flat_map(usize::from_str)
                            .collect();
                        Linear(ids[0], ids[1], ids[2])
                    }
                    _ => panic!("unrecognized curvil type {typ}"),
                });
            }
        }
        let pairs = zip(lxm_freqs, &ret.lxm).collect::<Vec<_>>();
        ret.lxm = pairs.iter().map(|p| p.1.clone()).collect();
        ret.compute_irreps();
        ret
    }

    fn compute_irreps(&mut self) {
        let pg = self.geom.point_group_approx(SYMM_EPS);
        for (i, disp) in self.lxm.iter().enumerate() {
            let mol = self.geom.clone() + disp.clone();
            let mut eps = SYMM_EPS;
            let mut irrep = mol.irrep_approx(&pg, eps);
            while let Err(e) = irrep {
                if eps >= 0.1 {
                    if DEBUG {
                        eprintln!(
                        "failed to compute irrep {i} for\n{}\nin {} with {e:?}",
                        mol, pg
                    );
                    }
                    // give up and give A
                    irrep = Ok(symm::Irrep::A);
                    break;
                }
                eps *= 10.0;
                if DEBUG {
                    eprintln!("warning: raising epsilon to {:.1e}", eps);
                }
                irrep = mol.irrep_approx(&pg, eps);
            }
            self.irreps.push(irrep.unwrap());
        }
    }
}

fn geom_handler(line: &str, state: &mut State, ret: &mut Summary) {
    let fields: Vec<_> = line.split_whitespace().collect();
    if fields.is_empty() {
        *state = State::None;
    } else {
        let atomic_number = match ATOMIC_WEIGHTS.get(fields[4]) {
            Some(a) => *a,
            None => {
                panic!("atom with weight {} not found, tell Brent!", fields[4])
            }
        };
        if let [x, y, z] = fields[1..=3]
            .iter()
            .map(|s| s.parse::<f64>().unwrap())
            .collect::<Vec<_>>()[..]
        {
            ret.geom.atoms.push(Atom::new(atomic_number, x, y, z));
        }
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nCartesian Geometry (Å): {:.8}", self.geom)?;

        writeln!(f, "Vibrational Frequencies (cm⁻¹):")?;
        writeln!(f, "ZPT = {:.1}", self.zpt)?;
        let width = f.width().unwrap_or(8);
        writeln!(
            f,
            "{:>5}{:>5}{:>width$}{:>width$}{:>width$}",
            "Mode",
            "Symm",
            "Harm",
            "Fund",
            "Corr",
            width = width
        )?;
        let prec = f.precision().unwrap_or(1);
        for i in 0..self.harm.len() {
            writeln!(
                f,
                "{:5}{:>5}{:width$.prec$}{:width$.prec$}{:width$.prec$}",
                i + 1,
                self.irreps.get(i).unwrap_or(&Irrep::A),
                self.harm.get(i).unwrap_or(&BAD_FLOAT),
                self.fund.get(i).unwrap_or(&BAD_FLOAT),
                self.corr.get(i).unwrap_or(&BAD_FLOAT),
                width = width,
                prec = prec,
            )?;
        }

        writeln!(f, "\nRotational Constants (cm⁻¹):")?;
        writeln!(f, "{:5}{:^15}{:^15}{:^15}", "State", "A", "B", "C")?;
        writeln!(
            f,
            "{:>5}{:15.7}{:15.7}{:15.7}",
            "e", self.rot_equil[0], self.rot_equil[1], self.rot_equil[2]
        )?;
        for (i, rot) in self.rots.iter().enumerate() {
            let mut v = rot.clone();
            // sort in descending order
            v.sort_by(|a, b| b.partial_cmp(a).unwrap());
            let (a, b, c) = (v[0], v[1], v[2]);
            writeln!(f, "{:5}{:15.7}{:15.7}{:15.7}", i, a, b, c)?;
        }

        writeln!(f, "\nQuartic Distortion Constants (MHz):")?;
        writeln!(f, "{}", self.deltas)?;

        writeln!(f, "\nSextic Distortion Constants (MHz):")?;
        writeln!(f, "{}", self.phis)?;

        Ok(())
    }
}

impl From<spectro::Output> for Summary {
    fn from(value: spectro::Output) -> Self {
        let rots = value
            .rots
            .iter()
            .map(|r| vec![TO_MHZ * r.a, TO_MHZ * r.b, TO_MHZ * r.c])
            .collect();
        let rot_equil = value.rot_equil.iter().map(to_mhz).collect();
        Self {
            harm: value.harms,
            fund: value.funds,
            corr: value.corrs,
            geom: value.geom,
            irreps: value.irreps,
            lxm: value.lxm,
            rots,
            rot_equil,
            deltas: value.quartic.into(),
            phis: value.sextic.into(),
            fermi: HashMap::new(),
            coriolis: Coriol::default(),
            zpt: value.zpt,
            // these are not computed by my spectro
            curvils: vec![],
            ralpha: vec![],
            requil: vec![],
        }
    }
}

impl From<spectro::quartic::Quartic> for Delta {
    fn from(value: spectro::quartic::Quartic) -> Self {
        Self {
            big_delta_j: Some(value.delj * TO_MHZ),
            big_delta_k: Some(value.delk * TO_MHZ),
            big_delta_jk: Some(value.deljk * TO_MHZ),
            delta_j: Some(value.sdelj * TO_MHZ),
            delta_k: Some(value.sdelk * TO_MHZ),
            d_j: Some(value.dj * TO_MHZ),
            d_jk: Some(value.djk * TO_MHZ),
            d_k: Some(value.dk * TO_MHZ),
            d1: Some(value.sd1 * TO_MHZ),
            d2: Some(value.sd2 * TO_MHZ),
            de: Some(value.de * TO_MHZ),
        }
    }
}

impl From<spectro::sextic::Sextic> for Phi {
    fn from(value: spectro::sextic::Sextic) -> Self {
        Self {
            big_phi_j: Some(value.phij * TO_MHZ),
            big_phi_k: Some(value.phik * TO_MHZ),
            big_phi_jk: Some(value.phijk * TO_MHZ),
            big_phi_kj: Some(value.phikj * TO_MHZ),
            phi_j: Some(value.sphij * TO_MHZ),
            phi_jk: Some(value.sphijk * TO_MHZ),
            phi_k: Some(value.sphik * TO_MHZ),
            h_j: Some(value.hj * TO_MHZ),
            h_jk: Some(value.hjk * TO_MHZ),
            h_kj: Some(value.hkj * TO_MHZ),
            h_k: Some(value.hk * TO_MHZ),
            h1: Some(value.h1 * TO_MHZ),
            h2: Some(value.h2 * TO_MHZ),
            h3: Some(value.h3 * TO_MHZ),
            he: Some(value.he * TO_MHZ),
        }
    }
}
