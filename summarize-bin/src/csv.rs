use summarize::{curvil::Curvil, Summary};

use crate::{format::Format, impl_display};

pub(crate) struct Csv {
    pub summaries: Vec<Summary>,
}

impl_display!(Csv);

impl Format for Csv {
    const SEP: &'static str = ",";

    fn len(&self) -> usize {
        self.summaries.len()
    }

    fn line(_: usize) -> String {
        String::from("")
    }

    max_fields! {
        harm => max_harms,
        corr => max_corrs,
        rots => max_rots,
    }

    fn curvil_label(&self, curvil: &Curvil, i: usize) -> String {
        use Curvil::*;
        let sum = &self.summaries[i];
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
            Opb(_, _, _, _) => todo!(),
        }
    }
}

impl<'a> IntoIterator for &'a Csv {
    type Item = &'a Summary;

    type IntoIter = std::slice::Iter<'a, Summary>;

    fn into_iter(self) -> Self::IntoIter {
        self.summaries.iter()
    }
}
