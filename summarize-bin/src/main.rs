use std::fmt::Display;

use summarize::Summary;

struct Text(Vec<Summary>);

impl Text {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// macro for generating max_* methods on `Text`, inspired by this
/// https://github.com/jonhoo/fantoccini/pull/186#discussion_r990712599
/// suggestion and by the actual implementation
/// https://github.com/jonhoo/fantoccini/pull/186/commits/8035a970e
macro_rules! max_fields {
    ($($field:ident => $name:ident$(,)?),*) => {
	$(
	    fn $name(&self) -> usize {
		self.0.iter().map(|sum| sum.$field.len()).max().unwrap()
	    }
	)*
    };
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
        writeln!(f, "Vibrational Frequencies (cm-1):")?;
        self.print_freqs(f)?;

        writeln!(f, "\nRotational Constants (cm-1):")?;
        self.print_rots(f)?;

        Ok(())
    }
}

impl Text {
    fn print_freqs(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        let max_harms = self.max_harms();
        let max_corrs = self.max_corrs();
        let nsum = self.len();
        // 3 for w/v label and 8 for each element in self
        let dashes = vec!["-"; 3 + 8 * nsum];
        let dashes = dashes.join("");
        for i in 0..max_harms {
            write!(f, "w{:<2}", i + 1)?;
            for sum in &self.0 {
                if let Some(v) = sum.harm.get(i) {
                    write!(f, "{:8.1}", v)?;
                } else {
                    write!(f, "{:8}", " ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "{}", dashes)?;
        write!(f, "ZPT")?;
        for sum in &self.0 {
            write!(f, "{:8.1}", sum.zpt)?;
        }
        writeln!(f)?;
        for i in 0..max_corrs {
            write!(f, "v{:<2}", i + 1)?;
            for sum in &self.0 {
                if let Some(v) = sum.corr.get(i) {
                    write!(f, "{:8.1}", v)?;
                } else {
                    write!(f, "{:8}", " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }

    fn print_rots(
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

        Ok(())
    }
}

fn main() {
    let infiles: Vec<_> = std::env::args().skip(1).collect();
    if infiles.is_empty() {
        panic!("usage: summarize FILENAME...");
    }
    let summaries: Vec<_> = infiles.iter().map(Summary::new).collect();

    println!("\n{}", Text(summaries));
}
