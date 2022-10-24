use std::fmt::Display;

use summarize::Summary;
use symm::Irrep;

#[allow(unused)]
pub enum TableType {
    Vib,
    Rot,
    DistA,
    DistS,
}

pub trait Format
where
    for<'a> &'a Self: IntoIterator<Item = &'a Summary>,
{
    /// separator between fields in a table
    const SEP: &'static str;

    /// end of row delimiter
    const END: &'static str;

    fn max_harms(&self) -> usize;
    fn max_corrs(&self) -> usize;
    fn max_rots(&self) -> usize;
    fn len(&self) -> usize;

    /// return the label for the harmonic frequencies. idx starts at 1
    fn omega(&self, idx: usize) -> String;

    /// return the label for the anharmonic frequencies. idx starts at 1
    fn nu(&self, idx: usize) -> String;

    /// return the desired format for an irrep
    fn irrep(&self, ir: &Irrep) -> String {
        ir.to_string()
    }

    fn sep(&self) -> &'static str {
        Self::SEP
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

    fn pre_table(&self, typ: TableType, cols: usize) -> String;

    fn post_table(&self) -> &'static str;

    /// combine the constant `c` and the subscript `sub` into a single
    /// rotational constant
    fn rot_const(&self, c: &str, sub: impl Display) -> String;

    /// returns the labels for the quartic distortion constants (deltas) in the
    /// order DeltaJ, DeltaK, DeltaJK, deltaJ, deltaK, DJ, DJK, DK, d1, d2
    fn delta_labels(&self) -> [&'static str; 10];

    /// returns the labels for the sextic distortion constants (phis) in the
    /// order PhiJ, PhiK, PhiJK, PhiKJ, phiJ, phiJK, phiK, HJ, HJK, HKJ, HK, h1,
    /// h2, h3
    fn phi_labels(&self) -> [&'static str; 14];

    /// helper method for writing the header for the distortion constant tables
    fn dist_header(
        &self,
        nsum: usize,
        f: &mut std::fmt::Formatter,
        dashes: impl Display,
    ) -> Result<(), std::fmt::Error> {
        if nsum > 1 {
            write!(f, "{:<13}{}", "Const.", self.sep())?;
            for i in 0..nsum {
                write!(
                    f,
                    r"{:>16} {}{}",
                    "Mol.",
                    i + 1,
                    self.end(i < nsum - 1)
                )
                .unwrap();
            }
        }
        writeln!(f, "\n{dashes}")?;
        Ok(())
    }

    fn print_freqs(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        let max_harms = self.max_harms();
        let max_corrs = self.max_corrs();
        let nsum = self.len();
        // 4 for w/v label, 6 for each symmetry label, and 8 for each frequency
        let dashes = Self::line(4 + 14 * nsum);

        writeln!(f, "{}", self.pre_table(TableType::Vib, 1 + 2 * nsum))?;

        write!(f, "Mode{}", Self::SEP)?;
        for i in 0..nsum {
            write!(
                f,
                "{:>6}{}{:>8}{}",
                "Symm.",
                Self::SEP,
                "Freq.",
                self.end(i < nsum - 1)
            )?;
        }
        writeln!(f, "\n{dashes}")?;

        for i in 0..max_harms {
            write!(f, " {}{}", self.omega(i + 1), Self::SEP)?;
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
        writeln!(f, "{}", dashes)?;
        write!(f, "ZPT{}", self.sep())?;
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
            write!(f, "{}{}", self.nu(i + 1), self.sep())?;
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
        if nsum > 1 {
            write!(f, "\nConst.{}", self.sep())?;
            for i in 0..nsum {
                write!(
                    f,
                    r"{:>14}{}{}",
                    "Mol. ",
                    i + 1,
                    self.end(i < nsum - 1)
                )
                .unwrap();
            }
        }
        writeln!(f)?;

        const WIDTH: usize = 15;
        const PREC: usize = 1;

        let dashes = Self::line(6 + WIDTH * nsum);
        writeln!(f, "{dashes}")?;

        // equilibrium
        for j in 0..3 {
            write!(f, "{}", self.rot_const(["A", "B", "C"][j], "e"))?;
            for (i, sum) in self.into_iter().enumerate() {
                if let Some(rot) = sum.rot_equil.get(j) {
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
                write!(f, "{}", self.rot_const(["A", "B", "C"][j], i))?;
                for (k, sum) in self.into_iter().enumerate() {
                    if let Some(rot) = sum.rots.get(i) {
                        if let Some(abc) = rot.get(j) {
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
        write!(f, "{:<6}{}", "k", self.sep())?;
        for (i, sum) in self.into_iter().enumerate() {
            if sum.rot_equil.len() == 3 {
                let r = &sum.rots[0];
                let (a, b, c) = (r[0], r[1], r[2]);
                let k = (2.0 * b - a - c) / (a - c);
                write!(f, "{:WIDTH$.7}", k)?;
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
        let dashes = Self::line(13 + 18 * nsum);

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
        }

        writeln!(f, "{}", self.post_table())?;

        Ok(())
    }

    fn print_curvils(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        writeln!(f, "\nCurvilinear Coordinates:\n")?;
        for (i, sum) in self.into_iter().enumerate() {
            writeln!(f, "Molecule {}\n", i + 1)?;
            writeln!(f, "{:^21}{:>18}{:>18}", "Coord", "R(EQUIL)", "R(ALPHA)")?;
            let vals = sum.requil.iter().zip(&sum.ralpha);
            for (curvil, (alpha, equil)) in sum.curvils.iter().zip(vals) {
                use summarize::curvil::Curvil::*;
                write!(
                    f,
                    "{:21}",
                    match curvil {
                        Bond(a, b) => format!(
                            "r({:>2}{a:<2} - {:>2}{b:<2})",
                            sum.geom.atoms[*a - 1].label(),
                            sum.geom.atoms[*b - 1].label()
                        ),
                        Angle(a, b, c) => format!(
                            "<({:>2}{a:<2} - {:>2}{b:<2} - {:>2}{c:<2})",
                            sum.geom.atoms[*a - 1].label(),
                            sum.geom.atoms[*b - 1].label(),
                            sum.geom.atoms[*c - 1].label()
                        ),
                        // pretty sure nothing else is printed in this part
                        Torsion(_, _, _, _) => todo!(),
                    }
                )?;
                writeln!(f, "{:18.7}{:18.7}", equil, alpha)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }

    fn print_fermi(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        writeln!(f, "\nFermi Resonances:\n")?;
        for (i, sum) in self.into_iter().enumerate() {
            writeln!(f, "Molecule {}\n", i + 1)?;
            let mut keys: Vec<_> = sum.fermi.keys().collect();
            keys.sort_unstable();
            for c in keys {
                for (a, b) in &sum.fermi[c] {
                    if a == b {
                        write!(f, "2w{a} = ")?;
                    } else {
                        write!(f, "w{a} + w{b} = ")?;
                    }
                }
                writeln!(f, "w{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }

    fn print_coriol(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        writeln!(f, "\nCoriolis Resonances:\n")?;
        for (i, sum) in self.into_iter().enumerate() {
            writeln!(f, "Molecule {}\n", i + 1)?;
            writeln!(f, "{:>8}{:>8}", "Modes", "Axes")?;
            let mut keys: Vec<_> = sum.coriolis.keys().collect();
            keys.sort_unstable();
            for c in keys {
                let (a, b) = c;
                // two spaces here and then max axes pads the rest of the room
                write!(f, "w{a:<2} = w{b:<2}  ")?;
                for axis in &sum.coriolis[c] {
                    write!(
                        f,
                        "{:>2}",
                        match axis {
                            3 => "C",
                            2 => "B",
                            1 => "A",
                            _ => "?",
                        }
                    )?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
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
                // self.print_freqs(f)?;
                // self.print_rots(f)?;
                // self.print_dist(f)?;

                self.print_curvils(f)?;
                self.print_fermi(f)?;
                self.print_coriol(f)?;

                Ok(())
            }
        }
    };
}
