use std::fmt::Display;

use summarize::Summary;

struct Text(Vec<Summary>);

impl Text {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_harms = self.0.iter().map(|sum| sum.harm.len()).max().unwrap();
        let max_corrs = self.0.iter().map(|sum| sum.corr.len()).max().unwrap();
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
}

fn main() {
    let infiles: Vec<_> = std::env::args().skip(1).collect();
    if infiles.is_empty() {
        panic!("usage: summarize FILENAME...");
    }
    let summaries: Vec<_> = infiles.iter().map(Summary::new).collect();

    println!("\n{}", Text(summaries));
}
