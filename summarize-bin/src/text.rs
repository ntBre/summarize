use summarize::{curvil::Curvil, Summary};

use crate::{
    format::{Format, TableType},
    impl_display,
};

pub(crate) struct Text(pub Vec<Summary>);

impl_display!(Text);

impl Format for Text {
    const SEP: &'static str = "";
    const END: &'static str = "";

    fn len(&self) -> usize {
        self.0.len()
    }

    fn line(width: usize) -> String {
        (vec!["-"; width]).join("")
    }

    max_fields! {
        harm => max_harms,
        corr => max_corrs,
        rots => max_rots,
    }

    fn omega(&self, idx: usize) -> String {
        format!("w{:<2}", idx)
    }

    fn nu(&self, idx: usize) -> String {
        format!("v{:<2}", idx)
    }

    fn pre_table(&self, typ: TableType, n: usize) -> String {
        match typ {
            TableType::Vib => String::from("Vibrational Frequencies (cm-1):\n"),
            TableType::Rot => {
                String::from("\nRotational Constants (in MHz):\n")
            }
            TableType::DistA => String::from(
                "\nQuartic and Sextic Distortion \
		 Constants in the Watson A-Reduced Hamiltonian (in MHz):\n",
            ),
            TableType::DistS => String::from(
                "\nQuartic and Sextic Distortion \
		 Constants in the Watson S-Reduced Hamiltonian (in MHz):\n",
            ),
            TableType::Curvil => {
                format!(
                    "Equilibrium and Vibrationally Averaged Curvilinear \
		     Coordinates for Mol. {} (in Å or °):\n",
                    n + 1
                )
            }
            TableType::Fermi => {
                format!("Fermi resonances for for Mol. {}:\n", n + 1)
            }
            TableType::Coriol => {
                format!(
                    "Coriolis resonances for for Mol. {}:

{:>8}{:>8}",
                    n + 1,
                    "Modes",
                    "Axes",
                )
            }
        }
    }

    fn post_table(&self) -> &'static str {
        ""
    }

    fn rot_const(&self, c: &str, sub: impl std::fmt::Display) -> String {
        format!("{}{:<5}{}", c, sub, self.sep())
    }

    fn delta_labels(&self) -> [&'static str; 10] {
        [
            "DELTA J", "DELTA K", "DELTA JK", "delta J", "delta K", "D J",
            "D JK", "D K", "d 1", "d 2",
        ]
    }

    fn phi_labels(&self) -> [&'static str; 14] {
        [
            "PHI J", "PHI K", "PHI JK", "PHI KJ", "phi j", "phi jk", "phi k",
            "H J", "H JK", "H KJ", "H K", "h 1", "h 2", "h 3",
        ]
    }

    fn curvil_label(&self, curvil: &Curvil, i: usize) -> String {
        use Curvil::*;
        let sum = &self.0[i];
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
    }
}

impl<'a> IntoIterator for &'a Text {
    type Item = &'a Summary;

    type IntoIter = std::slice::Iter<'a, Summary>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
