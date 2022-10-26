use summarize::{curvil::Curvil, Summary};

use crate::{
    format::{Format, TableType},
    impl_display,
};

pub(crate) struct Org(pub Vec<Summary>);

impl_display!(Org);

impl Format for Org {
    const PRE: &'static str = "|";
    const SEP: &'static str = "|";
    const END: &'static str = "|";

    fn len(&self) -> usize {
        self.0.len()
    }

    fn line(_: usize) -> String {
        String::from("|-")
    }

    max_fields! {
        harm => max_harms,
        corr => max_corrs,
        rots => max_rots,
    }

    fn pre_table(&self, typ: TableType, n: usize) -> String {
        match typ {
            TableType::Vib => String::from("#+name: vibs"),
            TableType::Rot => String::from("#+name: rots"),
            TableType::DistA => String::from("#+name: dista"),
            TableType::DistS => String::from("#+name: dists"),
            TableType::Curvil => {
                format!("#+name: curvils{}", n + 1)
            }
            TableType::Fermi => {
                format!("#+name: fermi{}", n + 1)
            }
            TableType::Coriol => {
                format!(
                    "#+name: coriol{}
|{:>8}|{:>8}|",
                    n + 1,
                    "Modes",
                    "Axes",
                )
            }
        }
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
            Linear(a, b, c) => format!(
                "LIN({:>2}{a:<2} - {:>2}{b:<2} - {:>2}{c:<2})",
                sum.geom.atoms[*a - 1].label(),
                sum.geom.atoms[*b - 1].label(),
                sum.geom.atoms[*c - 1].label()
            ),
        }
    }
}

impl<'a> IntoIterator for &'a Org {
    type Item = &'a Summary;

    type IntoIter = std::slice::Iter<'a, Summary>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
