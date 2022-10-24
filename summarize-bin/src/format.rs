use std::fmt::Display;

use summarize::Summary;
use symm::Irrep;

pub enum TableType {
    #[allow(unused)]
    Vib,
    #[allow(unused)]
    Rot,
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

    fn rot_const(&self, c: &str, sub: impl Display) -> String;

    /// returns the labels for the quartic distortion constants (deltas) in the
    /// order DeltaJ, DeltaK, DeltaJK, deltaJ, deltaK, DJ, DJK, DK, d1, d2
    fn delta_labels(&self) -> [&'static str; 10];
}

/// implement [std::fmt::Display] for a type that implements [Format]
#[macro_export]
macro_rules! impl_display {
    ($t:ty) => {
	impl ::std::fmt::Display for $t {
	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// self.print_freqs(f)?;

		// self.print_rots(f)?;

		writeln!(f, "\nA-Reduced Quartic Distortion Constants (MHz):\n")?;
		let labels = self.delta_labels();
		write_dist_consts! {
		    f, self, deltas,
		    big_delta_j => labels[0],
		    big_delta_k => labels[1],
		    big_delta_jk => labels[2],
		    delta_j => labels[3],
		    delta_k => labels[4],
		}

		writeln!(f, "\nS-Reduced Quartic Distortion Constants (MHz):\n")?;
		write_dist_consts! {
		    f, self, deltas,
		    d_j => labels[5],
		    d_jk => labels[6],
		    d_k => labels[7],
		    d1 => labels[8],
		    d2 => labels[9],
		}

		writeln!(f, "\nA-Reduced Sextic Distortion Constants (MHz):\n")?;
		write_dist_consts! {
		    f, self, phis,
		    big_phi_j => "PHI J",
		    big_phi_k => "PHI K",
		    big_phi_jk => "PHI JK",
		    big_phi_kj => "PHI KJ",
		    phi_j => "phi j",
		    phi_jk => "phi jk",
		    phi_k => "phi k",
		}

		writeln!(f, "\nS-Reduced Sextic Distortion Constants (MHz):\n")?;
		write_dist_consts! {
		    f, self, phis,
		    h_j => "H J",
		    h_jk => "H JK",
		    h_kj => "H KJ",
		    h_k => "H K",
		    h1 => "h 1",
		    h2 => "h 2",
		    h3 => "h 3",
		}

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
    };
}
