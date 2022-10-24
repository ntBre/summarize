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
        match typ {
            TableType::Vib => {
                // left align mode column followed
                let mut s = String::from("l");
                for _ in 0..cols / 2 {
                    s.push_str("lr");
                }
                let head = if cols / 2 > 1 {
                    use std::fmt::Write;
                    let mut h = String::from("\n & ");
                    for i in 0..cols / 2 {
                        write!(
                            h,
                            r"\multicolumn{{2}}{{c}}{{Mol. {}}}{}",
                            i + 1,
                            self.end(i < cols / 2 - 1)
                        )
                        .unwrap();
                    }
                    h
                } else {
                    "".to_owned()
                };
                let cap = "Vibrational frequencies (in cm$^{-1}$)";
                format!(
                    r"\begin{{table}}
\centering
\caption{{{cap}}}
\begin{{tabular}}{{{s}}}{head}",
                )
            }
            TableType::Rot => {
                let mut s = String::from("l");
                for _ in 0..cols {
                    s.push('r');
                }
                let cap = "Rotational Constants (in MHz)";
                format!(
                    r"\begin{{table}}
\centering
\caption{{{cap}}}
\begin{{tabular}}{{{s}}}",
                )
            }
            TableType::DistA | TableType::DistS => {
                let cap = format!(
                    "Quartic and sextic distortion constants in the \
				   Watson {}-reduced Hamiltonian (in MHz)",
                    if matches!(typ, TableType::DistA) {
                        "A"
                    } else {
                        "S"
                    }
                );
                let mut s = String::from("l");
                for _ in 0..cols {
                    s.push('r');
                }
                format!(
                    r"\begin{{table}}
\centering
\caption{{{cap}}}
\begin{{tabular}}{{{s}}}",
                )
            }
        }
    }

    fn post_table(&self) -> &'static str {
        r"\end{tabular}
\end{table}"
    }

    fn rot_const(&self, c: &str, sub: impl std::fmt::Display) -> String {
        format!("${}_{{{:<5}}}${}", c, sub, self.sep())
    }

    fn delta_labels(&self) -> [&'static str; 10] {
        [
            r"$\Delta_{J}$",
            r"$\Delta_{K}$",
            r"$\Delta_{JK}$",
            r"$\delta_{J}$",
            r"$\delta_{K}$",
            r"$D_{J}$",
            r"$D_{JK}$",
            r"$D_{K}$",
            r"$d_{1}$",
            r"$d_{2}$",
        ]
    }

    fn phi_labels(&self) -> [&'static str; 14] {
        [
            r"$\Phi_{J}$",
            r"$\Phi_{K}$",
            r"$\Phi_{JK}$",
            r"$\Phi_{KJ}$",
            r"$\phi_{j}$",
            r"$\phi_{jk}$",
            r"$\phi_{k}$",
            r"$H_{J}$",
            r"$H_{JK}$",
            r"$H_{KJ}$",
            r"$H_{K}$",
            r"$h_{1}$",
            r"$h_{2}$",
            r"$h_{3}$",
        ]
    }
}

impl<'a> IntoIterator for &'a Latex {
    type Item = &'a Summary;

    type IntoIter = std::slice::Iter<'a, Summary>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
