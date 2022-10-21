use std::fmt::Display;

use summarize::Summary;

pub(crate) struct Text(pub Vec<Summary>);

impl Text {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl Text {
    max_fields! {
        harm => max_harms,
        corr => max_corrs,
        rots => max_rots,
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Vibrational Frequencies (cm-1):\n")?;
        self.print_freqs(f)?;

        writeln!(f, "\nRotational Constants (cm-1):\n")?;
        self.print_rots(f)?;

        writeln!(f, "\nQuartic Distortion Constants (MHz):\n")?;
        write_dist_consts! {
            f, &self.0, deltas,
            big_delta_j => "DELTA J",
            big_delta_k => "DELTA K",
            big_delta_jk => "DELTA JK",
            delta_j => "delta J",
            delta_k => "delta K",
        }

        writeln!(f, "\nSextic Distortion Constants (MHz):\n")?;
        write_dist_consts! {
            f, &self.0, phis,
            big_phi_j => "PHI J",
            big_phi_k => "PHI K",
            big_phi_jk => "PHI JK",
            big_phi_kj => "PHI KJ",
            phi_j => "phi j",
            phi_jk => "phi jk",
            phi_k => "phi k",
        }

        writeln!(f, "\nCurvilinear Coordinates:\n")?;
        for (i, sum) in self.0.iter().enumerate() {
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
        }

        writeln!(f, "\nFermi Resonances:\n")?;
        for (i, sum) in self.0.iter().enumerate() {
            writeln!(f, "Molecule {}", i + 1)?;
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

        writeln!(f, "\nCoriolis Resonances:\n")?;
        for (i, sum) in self.0.iter().enumerate() {
            writeln!(f, "Molecule {}", i + 1)?;
            writeln!(f, "{:>8}{:>8}", "Modes", "Axes")?;
            // 16 is the sum of the eights above
            writeln!(f, "{:->16}", "")?;
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

impl Text {
    pub(crate) fn print_freqs(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        let max_harms = self.max_harms();
        let max_corrs = self.max_corrs();
        let nsum = self.len();
        // 4 for w/v label, 6 for each symmetry label, and 8 for each frequency
        let dashes = vec!["-"; 4 + 14 * nsum];
        let dashes = dashes.join("");

        write!(f, "Mode")?;
        for _ in 0..nsum {
            write!(f, "{:>6}{:>8}", "Symm", "Freq.")?;
        }
        writeln!(f, "\n{dashes}")?;

        for i in 0..max_harms {
            write!(f, " w{:<2}", i + 1)?;
            for sum in &self.0 {
                if let Some(v) = sum.harm.get(i) {
                    write!(
                        f,
                        "{:>6}{:8.1}",
                        sum.irreps.get(i).unwrap_or(&symm::Irrep::A),
                        v
                    )?;
                } else {
                    write!(f, "{:14}", " ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "{}", dashes)?;
        write!(f, "ZPT ")?;
        for sum in &self.0 {
            write!(f, "{:14.1}", sum.zpt)?;
        }
        writeln!(f)?;

        for i in 0..max_corrs {
            write!(f, " v{:<2}", i + 1)?;
            for sum in &self.0 {
                if let Some(v) = sum.corr.get(i) {
                    write!(
                        f,
                        "{:>6}{:8.1}",
                        sum.irreps.get(i).unwrap_or(&symm::Irrep::A),
                        v
                    )?;
                } else {
                    write!(f, "{:14}", " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }

    pub(crate) fn print_rots(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        // equilibrium
        for j in 0..3 {
            write!(f, "{}e ", ["A", "B", "C"][j])?;
            for sum in &self.0 {
                if let Some(rot) = sum.rot_equil.get(j) {
                    write!(f, "{:15.7}", rot)?;
                } else {
                    write!(f, "{:15}", "")?;
                }
            }
            writeln!(f)?;
        }

        for i in 0..self.max_rots() {
            // loop over a, b, and c
            for j in 0..3 {
                write!(f, "{}{:<2}", ["A", "B", "C"][j], i)?;
                for sum in &self.0 {
                    if let Some(rot) = sum.rots.get(i) {
                        if let Some(abc) = rot.get(j) {
                            write!(f, "{:15.7}", abc)?;
                        } else {
                            write!(f, "{:15}", "")?;
                        }
                    } else {
                        write!(f, "{:15}", "")?;
                    }
                }
                writeln!(f)?;
            }
        }

        // this is kappa for the vibrationally-averaged rotational constants
        write!(f, "k  ")?;
        for sum in &self.0 {
            if sum.rot_equil.len() == 3 {
                let r = &sum.rots[0];
                let (a, b, c) = (r[0], r[1], r[2]);
                let k = (2.0 * b - a - c) / (a - c);
                write!(f, "{:15.7}", k)?;
            } else {
                write!(f, "{:15.7}", "")?;
            }
        }
        writeln!(f)?;

        Ok(())
    }
}
