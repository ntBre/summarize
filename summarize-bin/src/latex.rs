use summarize::Summary;

use crate::{
    format::{Format, TableType},
    impl_display,
};

pub(crate) struct Latex(pub Vec<Summary>);

impl_display!(Latex);

impl Format for Latex {
    const SEP: &'static str = " & ";
    const END: &'static str = r" \\";

    fn len(&self) -> usize {
        self.0.len()
    }

    fn omega(&self, idx: usize) -> String {
        format!(r"$\omega_{{{:<2}}}$", idx)
    }

    fn nu(&self, idx: usize) -> String {
        format!(r"$\nu_{{{:<2}}}$", idx)
    }

    fn line(_: usize) -> String {
        String::from(r"\hline")
    }

    fn irrep(&self, ir: &symm::Irrep) -> String {
        format!(
            "${:>6}$",
            match ir {
                symm::Irrep::A => "a",
                symm::Irrep::B => "b",
                symm::Irrep::Ap => "a'",
                symm::Irrep::App => "a''",
                symm::Irrep::A1 => "a_1",
                symm::Irrep::B2 => "b_2",
                symm::Irrep::B1 => "b_1",
                symm::Irrep::A2 => "a_2",
                symm::Irrep::Ag => "a_g",
                symm::Irrep::B1g => "b_{1g}",
                symm::Irrep::B2g => "b_{2g}",
                symm::Irrep::B3g => "b_{3g}",
                symm::Irrep::Au => "a_u",
                symm::Irrep::B1u => "b_{1u}",
                symm::Irrep::B2u => "b_{2u}",
                symm::Irrep::B3u => "b_{3u}",
                symm::Irrep::A1p => "a_1'",
                symm::Irrep::A2p => "a_2'",
                symm::Irrep::Ep => "e'",
                symm::Irrep::A1pp => "a_1''",
                symm::Irrep::A2pp => "a_2''",
                symm::Irrep::Epp => "e''",
            }
        )
    }

    max_fields! {
        harm => max_harms,
        corr => max_corrs,
        rots => max_rots,
    }

    fn pre_table(&self, typ: TableType, cols: usize) -> String {
        let (cap, fmt) = match typ {
            TableType::Vib => {
                // left align mode column followed
                let mut s = String::from("l");
                for _ in 0..cols / 2 {
                    s.push_str("lr");
                }
                ("Vibrational frequencies (in cm$^{-1}$)", s)
            }
        };
        format!(
            r"\begin{{table}}
\centering
\caption{{{cap}}}
\begin{{tabular}}{{{fmt}}}",
        )
    }

    fn post_table(&self) -> &'static str {
        r"\end{tabular}
\end{table}"
    }
}

impl<'a> IntoIterator for &'a Latex {
    type Item = &'a Summary;

    type IntoIter = std::slice::Iter<'a, Summary>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
