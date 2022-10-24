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
