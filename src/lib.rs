use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader},
    iter::zip,
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;
use symm::{Atom, Irrep, Molecule};

#[cfg(test)]
mod tests;

const BAD_FLOAT: f64 = 999999999.9;

/// threshold for discarding rotations and translations
const ROTRANS_THRSH: f64 = 30.0;

/// threshold for computing irrep symmetries
const SYMM_EPS: f64 = 1e-4;

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
}

#[derive(Default, Debug, PartialEq)]
pub struct Summary {
    pub harm: Vec<f64>,
    pub fund: Vec<f64>,
    pub corr: Vec<f64>,
    // these fields are needed to get the symmetries of the modes
    pub geom: Molecule,
    pub irreps: Vec<Irrep>,
    pub lxm: Vec<Vec<f64>>,
    pub rots: Vec<Vec<f64>>,
    // pub deltas: Vec<f64>,
    // pub phis: Vec<f64>,
    // pub rhead: Vec<String>,
    // pub ralpha: Vec<f64>,
    // pub requil: Vec<f64>,
    // pub fermi: Vec<String>,
    pub zpt: f64,
    // pub lin: bool,
    // pub imag: bool,
}

#[derive(PartialEq)]
enum State {
    Fund,
    Corr,
    Geom,
    LXM,
    Rots,
    None,
}

impl Summary {
    pub fn new(filename: &str) -> Self {
        let f = match std::fs::File::open(filename) {
            Ok(f) => f,
            Err(e) => panic!("failed to open {} with '{}'", filename, e),
        };
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
                state = State::LXM;
                // reset these. for degmodes it gets printed twice
                block = 0;
                lxm_freqs = Vec::new();
                ret.lxm = Vec::new();
            } else if state == State::LXM {
                let fields: Vec<_> = line.split_whitespace().collect();
                if fields.len() == 0 {
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
                && vib_states.len() > 0
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
                state = State::Rots;
                rot_good = true;
            } else if rot_good && line.contains("BZA") {
                state = State::Rots;
            } else if state == State::Rots && rot_good {
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
                ret.rots.push(
                    fields
                        .iter()
                        .map(|s| s.parse().unwrap_or(BAD_FLOAT))
                        .collect(),
                );
            }
        }
        let mut pairs = zip(lxm_freqs, &ret.lxm).collect::<Vec<_>>();
        pairs.dedup_by(|a, b| a.0 == b.0);
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
            while let Err(_) = irrep {
                if eps >= 0.1 {
                    panic!(
                        "failed to compute irrep {i} for\n{}\nin {}",
                        mol, pg
                    )
                }
                eps *= 10.0;
                eprintln!("warning: raising epsilon to {:.1e}", eps);
                irrep = mol.irrep_approx(&pg, eps);
            }
            self.irreps.push(irrep.unwrap());
        }
    }
}

fn geom_handler(line: &String, state: &mut State, ret: &mut Summary) {
    let fields: Vec<_> = line.split_whitespace().collect();
    if fields.len() == 0 {
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
                self.irreps[i],
                self.harm[i],
                self.fund[i],
                self.corr[i],
                width = width,
                prec = prec,
            )?;
        }
        Ok(())
    }
}
