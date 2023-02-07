use std::fmt::Display;

use summarize::{curvil::Curvil, Summary};
use symm::Irrep;

#[allow(unused)]
pub enum TableType {
    Vib,
    Rot,
    DistA,
    DistS,
    Curvil,
    Fermi,
    Coriol,
}

pub trait Format
where
    for<'a> &'a Self: IntoIterator<Item = &'a Summary>,
{
    /// beginning of row delimiter
    const PRE: &'static str = "";

    /// separator between fields in a table
    const SEP: &'static str = "";

    /// end of row delimiter
    const END: &'static str = "";

    fn max_harms(&self) -> usize;
    fn max_corrs(&self) -> usize;
    fn max_rots(&self) -> usize;
    fn len(&self) -> usize;

    /// return the label for the harmonic frequencies. idx starts at 1
    fn omega(&self, idx: usize) -> String {
        format!("w{idx:<2}")
    }

    /// return the label for the anharmonic frequencies. idx starts at 1
    fn nu(&self, idx: usize) -> String {
        format!("v{idx:<2}")
    }

    /// return the desired format for an irrep
    fn irrep(&self, ir: &Irrep) -> String {
        ir.to_string()
    }

    fn sep(&self) -> &'static str {
        Self::SEP
    }

    fn pre(&self) -> &'static str {
        Self::PRE
    }

    fn end(&self, end: bool) -> &'static str {
        if !end {
            Self::END
        } else {
            Self::SEP
        }
    }

    /// function for including horizontal lines in the output
    fn line(width: usize) -> String;

    /// called before each table
    fn pre_table(&self, typ: TableType, n: usize) -> String {
        match typ {
            TableType::Vib => String::from("Vibrational Frequencies (cm-1):\n"),
            TableType::Rot => {
                String::from("\nRotational Constants (in MHz):\n")
            }
            TableType::DistA => String::from(
                "\nQuartic and Sextic Distortion \
		 Constants in the Watson A-Reduced Hamiltonian:\n",
            ),
            TableType::DistS => String::from(
                "\nQuartic and Sextic Distortion \
		 Constants in the Watson S-Reduced Hamiltonian:\n",
            ),
            TableType::Curvil => {
                format!(
                    "Equilibrium and Vibrationally Averaged Curvilinear \
		     Coordinates for Mol. {} (in Å or °):\n",
                    n + 1
                )
            }
            TableType::Fermi => {
                format!("Fermi resonances for Mol. {}:\n", n + 1)
            }
            TableType::Coriol => {
                format!(
                    "Coriolis resonances for Mol. {}:

{:>8}{:>8}",
                    n + 1,
                    "Modes",
                    "Axes",
                )
            }
        }
    }

    /// called after each table
    fn post_table(&self) -> &'static str {
        ""
    }

    /// combine the constant `c` and the subscript `sub` into a single
    /// rotational constant, followed by `Self::SEP`
    fn rot_const(&self, c: &str, sub: impl std::fmt::Display) -> String {
        format!("{}{:<5}{}", c, sub, self.sep())
    }

    /// returns the labels for the quartic distortion constants (deltas) in the
    /// order DeltaJ, DeltaK, DeltaJK, deltaJ, deltaK, DJ, DJK, DK, d1, d2
    fn delta_labels(&self) -> [&'static str; 11] {
        [
            "DELTA J", "DELTA K", "DELTA JK", "delta J", "delta K", "D J",
            "D JK", "D K", "d 1", "d 2", "De",
        ]
    }

    /// returns the labels for the sextic distortion constants (phis) in the
    /// order PhiJ, PhiK, PhiJK, PhiKJ, phiJ, phiJK, phiK, HJ, HJK, HKJ, HK, h1,
    /// h2, h3
    fn phi_labels(&self) -> [&'static str; 15] {
        [
            "PHI J", "PHI K", "PHI JK", "PHI KJ", "phi j", "phi jk", "phi k",
            "H J", "H JK", "H KJ", "H K", "h 1", "h 2", "h 3", "He",
        ]
    }

    /// helper method for writing the header for the distortion constant tables
    fn dist_header(
        &self,
        nsum: usize,
        f: &mut std::fmt::Formatter,
        dashes: impl Display,
    ) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}{:<13}{}{:<8}{}",
            self.pre(),
            "Const.",
            self.sep(),
            "Units",
            self.sep()
        )?;
        for i in 0..nsum {
            write!(f, r"{:>8} {}{}", "Mol.", i + 1, self.end(i < nsum - 1))
                .unwrap();
        }
        writeln!(f, "\n{dashes}")?;
        Ok(())
    }

    fn format_dist_unit(&self, unit: crate::Unit) -> String {
        String::from(match unit {
            crate::Unit::uHz => "μHz",
            crate::Unit::mHz => "mHz",
            crate::Unit::Hz => "Hz",
            crate::Unit::kHz => "kHz",
            crate::Unit::MHz => "MHz",
            crate::Unit::GHz => "GHz",
            crate::Unit::THz => "THz",
        })
    }

    fn print_freqs(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        let max_harms = self.max_harms();
        let max_corrs = self.max_corrs();
        let nsum = self.len();
        // 4 for w/v label, 6 for each symmetry label, and 8 for each frequency
        let dashes = Self::line(4 + 16 * nsum);

        writeln!(f, "{}", self.pre_table(TableType::Vib, 1 + 2 * nsum))?;

        write!(f, "{}Mode{}", self.pre(), self.sep())?;
        for i in 0..nsum {
            write!(
                f,
                "{:>8}{}{:>8}{}",
                "Symm.",
                Self::SEP,
                "Freq.",
                self.end(i < nsum - 1)
            )?;
        }
        writeln!(f, "\n{dashes}")?;

        for i in 0..max_harms {
            write!(f, "{} {}{}", self.pre(), self.omega(i + 1), Self::SEP)?;
            for (j, sum) in self.into_iter().enumerate() {
                if let Some(v) = sum.harm.get(i) {
                    write!(
                        f,
                        "{:>8}{}{:8.1}{}",
                        self.irrep(
                            sum.irreps.get(i).unwrap_or(&symm::Irrep::A),
                        ),
                        self.sep(),
                        v,
                        self.end(j < nsum - 1)
                    )?;
                } else {
                    write!(
                        f,
                        "{:8}{}{:8}{}",
                        "",
                        self.sep(),
                        "",
                        self.end(j < nsum - 1)
                    )?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "{dashes}")?;
        write!(f, "{} ZPT{}", self.pre(), self.sep())?;
        for (i, sum) in self.into_iter().enumerate() {
            write!(
                f,
                "{:8}{}{:8.1}{}",
                "",
                self.sep(),
                sum.zpt,
                self.end(i < nsum - 1)
            )?;
        }
        writeln!(f)?;

        for i in 0..max_corrs {
            write!(f, "{} {}{}", self.pre(), self.nu(i + 1), self.sep())?;
            for (j, sum) in self.into_iter().enumerate() {
                if let Some(v) = sum.corr.get(i) {
                    write!(
                        f,
                        "{:>8}{}{:8.1}{}",
                        self.irrep(
                            sum.irreps.get(i).unwrap_or(&symm::Irrep::A),
                        ),
                        self.sep(),
                        v,
                        self.end(j < nsum - 1)
                    )?;
                } else {
                    write!(
                        f,
                        "{:8}{}{:8}{}",
                        "",
                        self.sep(),
                        "",
                        self.end(j < nsum - 1)
                    )?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "{}", self.post_table())?;

        Ok(())
    }

    fn print_rots(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        let nsum = self.len();
        write!(f, "{}", self.pre_table(TableType::Rot, 1 + nsum))?;
        write!(
            f,
            "\n{}Const.{}{:>8}{}",
            self.pre(),
            self.sep(),
            "Units",
            self.sep()
        )?;
        for i in 0..nsum {
            write!(f, r"{:>14}{}{}", "Mol. ", i + 1, self.end(i < nsum - 1))
                .unwrap();
        }
        writeln!(f)?;

        const WIDTH: usize = 15;
        const PREC: usize = 1;

        let dashes = Self::line(6 + 8 + WIDTH * nsum);
        writeln!(f, "{dashes}")?;

        // equilibrium
        for j in 0..3 {
            // apparently rot_const gives me a sep anyway
            write!(
                f,
                "{}{}{:>8}{}",
                self.pre(),
                self.rot_const(["A", "B", "C"][j], "e"),
                "MHz",
                self.sep(),
            )?;
            for (i, sum) in self.into_iter().enumerate() {
                // linear molecule - only have B
                let rot = if sum.rot_equil.len() == 1 {
                    if j == 1 {
                        sum.rot_equil.first()
                    } else {
                        None
                    }
                } else if sum.rot_equil.len() == 2 {
                    // symmetric top - only have A and C
                    if j == 0 {
                        sum.rot_equil.first()
                    } else if j == 2 {
                        sum.rot_equil.last()
                    } else {
                        None
                    }
                } else {
                    sum.rot_equil.get(j)
                };
                if let Some(rot) = rot {
                    write!(
                        f,
                        "{:WIDTH$.PREC$}{}",
                        rot,
                        self.end(i < nsum - 1)
                    )?;
                } else {
                    write!(f, "{:WIDTH$}{}", "", self.end(i < nsum - 1))?;
                }
            }
            writeln!(f)?;
        }

        for i in 0..self.max_rots() {
            // loop over a, b, and c
            for j in 0..3 {
                write!(
                    f,
                    "{}{}{:>8}{}",
                    self.pre(),
                    self.rot_const(["A", "B", "C"][j], i),
                    "MHz",
                    self.sep(),
                )?;
                for (k, sum) in self.into_iter().enumerate() {
                    if let Some(rot) = sum.rots.get(i) {
                        let abc = if rot.len() == 1 {
                            if j == 1 {
                                rot.first()
                            } else {
                                None
                            }
                        } else if rot.len() == 2 {
                            if j == 0 {
                                rot.first()
                            } else if j == 2 {
                                rot.last()
                            } else {
                                None
                            }
                        } else {
                            rot.get(j)
                        };
                        if let Some(abc) = abc {
                            write!(
                                f,
                                "{:WIDTH$.PREC$}{}",
                                abc,
                                self.end(k < nsum - 1)
                            )?;
                        } else {
                            write!(
                                f,
                                "{:WIDTH$.PREC$}{}",
                                "",
                                self.end(k < nsum - 1)
                            )?;
                        }
                    } else {
                        write!(
                            f,
                            "{:WIDTH$.PREC$}{}",
                            "",
                            self.end(k < nsum - 1)
                        )?;
                    }
                }
                writeln!(f)?;
            }
        }

        // this is kappa for the vibrationally-averaged rotational constants
        write!(
            f,
            "{}{:<6}{}{:8}{}",
            self.pre(),
            "k",
            self.sep(),
            "",
            self.sep()
        )?;
        for (i, sum) in self.into_iter().enumerate() {
            if sum.rot_equil.len() == 3 {
                let r = &sum.rots[0];
                let (a, b, c) = (r[0], r[1], r[2]);
                let k = (2.0 * b - a - c) / (a - c);
                write!(f, "{k:WIDTH$.7}")?;
            } else {
                write!(f, "{:WIDTH$.7}", "")?;
            }
            write!(f, "{}", self.end(i < nsum - 1))?;
        }
        writeln!(f)?;

        writeln!(f, "{}", self.post_table())?;

        Ok(())
    }

    fn print_dist(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        let nsum = self.len();
        writeln!(f, "{}", self.pre_table(TableType::DistA, 1 + nsum))?;
        let dashes = Self::line(13 + 8 + 10 * nsum);

        self.dist_header(nsum, f, &dashes)?;

        let delta_labels = self.delta_labels();
        write_dist_consts! {
            f, self, deltas,
            big_delta_j => delta_labels[0],
            big_delta_k => delta_labels[1],
            big_delta_jk => delta_labels[2],
            delta_j => delta_labels[3],
            delta_k => delta_labels[4],
        }

        writeln!(f, "{dashes}")?;

        let phi_labels = self.phi_labels();
        write_dist_consts! {
            f, self, phis,
            big_phi_j =>  phi_labels[0],
            big_phi_k =>  phi_labels[1],
            big_phi_jk =>  phi_labels[2],
            big_phi_kj =>  phi_labels[3],
            phi_j =>  phi_labels[4],
            phi_jk =>  phi_labels[5],
            phi_k =>  phi_labels[6],
        }

        writeln!(f, "{}\n", self.post_table())?;

        writeln!(f, "{}", self.pre_table(TableType::DistS, 1 + nsum))?;
        self.dist_header(nsum, f, &dashes)?;

        write_dist_consts! {
            f, self, deltas,
            d_j => delta_labels[5],
            d_jk => delta_labels[6],
            d_k => delta_labels[7],
            d1 => delta_labels[8],
            d2 => delta_labels[9],
        de => delta_labels[10],
        }

        writeln!(f, "{dashes}")?;

        write_dist_consts! {
            f, self, phis,
            h_j =>  phi_labels[7],
            h_jk =>  phi_labels[8],
            h_kj =>  phi_labels[9],
            h_k =>  phi_labels[10],
            h1 =>  phi_labels[11],
            h2 =>  phi_labels[12],
            h3 =>  phi_labels[13],
        he => phi_labels[14],
        }

        writeln!(f, "{}\n", self.post_table())?;

        Ok(())
    }

    fn curvil_label(&self, curvil: &Curvil, i: usize) -> String;

    fn print_curvils(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        const FIRST: usize = 21;
        const AFTER: usize = 18;
        let dashes = Self::line(FIRST + AFTER * 2);
        for (i, sum) in self.into_iter().enumerate() {
            writeln!(f, "{}", self.pre_table(TableType::Curvil, i))?;
            writeln!(
                f,
                "{}{:^FIRST$}{}{:>AFTER$}{}{:>AFTER$}{}",
                self.pre(),
                "Coord.",
                self.sep(),
                "Equil.",
                self.sep(),
                "Vib. Avg.",
                self.end(false)
            )?;
            writeln!(f, "{dashes}")?;
            let vals = sum.requil.iter().zip(&sum.ralpha);
            for (curvil, (alpha, equil)) in sum.curvils.iter().zip(vals) {
                write!(
                    f,
                    "{}{:FIRST$}{}",
                    self.pre(),
                    self.curvil_label(curvil, i),
                    self.sep()
                )?;
                // use this width to control for longer labels of some coords
                let (width, prec) = match curvil {
                    Curvil::Bond(_, _) => (AFTER, 5),
                    Curvil::Angle(_, _, _) => (AFTER, 3),
                    Curvil::Linear(_, _, _) => (AFTER - 2, 3),
                    Curvil::Torsion(_, _, _, _) => todo!(),
                    Curvil::Opb(_, _, _, _) => todo!(),
                };
                writeln!(
                    f,
                    "{:width$.prec$}{}{:AFTER$.prec$}{}",
                    equil,
                    self.sep(),
                    alpha,
                    self.end(false)
                )?;
            }
            writeln!(f, "{}\n", self.post_table())?;
        }

        Ok(())
    }

    fn print_fermi(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        for (i, sum) in self.into_iter().enumerate() {
            writeln!(f, "{}", self.pre_table(TableType::Fermi, i))?;
            let mut keys: Vec<_> = sum.fermi.keys().collect();
            keys.sort_unstable();
            for c in keys {
                write!(f, "{}", self.pre())?;
                for (a, b) in &sum.fermi[c] {
                    if a == b {
                        write!(f, "2{} = ", self.omega(*a))?;
                    } else {
                        write!(
                            f,
                            "{} + {} = ",
                            self.omega(*a),
                            self.omega(*b)
                        )?;
                    }
                }
                writeln!(f, "{}{}", self.omega(*c), self.end(false))?;
            }
            writeln!(f, "{}\n", self.post_table())?;
        }

        Ok(())
    }

    fn print_coriol(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        let dashes = Self::line(16);
        for (i, sum) in self.into_iter().enumerate() {
            writeln!(f, "{}", self.pre_table(TableType::Coriol, i))?;
            writeln!(f, "{dashes}")?;
            let mut keys: Vec<_> = sum.coriolis.data.keys().collect();
            keys.sort_unstable();
            for c in keys {
                let (a, b) = c;
                // two spaces here and then max axes pads the rest of the room
                write!(
                    f,
                    "{}{} = {}  {}",
                    self.pre(),
                    self.omega(*a),
                    self.omega(*b),
                    self.sep()
                )?;
                for axis in &sum.coriolis.data[c] {
                    write!(
                        f,
                        "{:>2}{}",
                        match axis {
                            3 => "C",
                            2 => "B",
                            1 => "A",
                            _ => "?",
                        },
                        self.end(false),
                    )?;
                }
                writeln!(f)?;
            }
            writeln!(f, "{}\n", self.post_table())?;
        }

        Ok(())
    }
}

/// implement [std::fmt::Display] for a type that implements [Format]
#[macro_export]
macro_rules! impl_display {
    ($t:ty) => {
        impl ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.print_freqs(f)?;
                self.print_rots(f)?;
                self.print_dist(f)?;
                self.print_curvils(f)?;
                self.print_fermi(f)?;
                self.print_coriol(f)?;

                Ok(())
            }
        }
    };
}
