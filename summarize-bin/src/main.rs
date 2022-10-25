use clap::Parser;

use summarize::Summary;

use crate::{latex::Latex, text::Text};

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

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Unit {
    uHz,
    mHz,
    Hz,
    kHz,
    MHz,
    GHz,
    THz,
}

/// find the optimal units for displaying distortion constants with 5 sig figs
/// and convert `vals` to those units
fn find_units(vals: Vec<Option<f64>>) -> (Vec<Option<f64>>, Unit) {
    /// multiplicative value/unit pairs for converting from MHz
    const UNITS: [(f64, Unit); 6] = [
        //
        (1e9, Unit::mHz),
        (1e6, Unit::Hz),
        (1e3, Unit::kHz),
        (1e0, Unit::MHz),
        (1e-3, Unit::GHz),
        (1e-6, Unit::THz),
    ];
    for (u, s) in UNITS {
        // want the smallest unit for which every v is < 1000.0
        if vals
            .iter()
            .flatten()
            .map(|v| v * u)
            .all(|a| a.abs() < 1000.0)
        {
            return (vals.iter().map(|v| v.map(|v| v * u)).collect(), s);
        }
    }
    (vals, Unit::MHz)
}

macro_rules! write_dist_consts {
    ($w:ident, $iter: expr, $struct:ident,
     $($field:ident => $name:expr$(,)?),*) => {
	$(
	    let nsum = $iter.len();
	    let vals = $iter.into_iter().map(|sum| sum.$struct.$field).collect();
	    let (vals, unit) = crate::find_units(vals);
	    write!($w, "{:<13}{}{:<8}{}", $name, $iter.sep(), $iter.format_dist_unit(unit), $iter.sep())?;
	    for (i, v) in vals.iter().enumerate() {
		if let Some(d) = v {
		    write!($w, "{:10.3}", d)?;
		} else {
		    write!($w, "{:10.3}", "")?;
		}
		write!($w, "{}", $iter.end(i < nsum-1))?;
	    }

	    writeln!($w)?;
	)*
    };
}

mod format;
mod latex;
mod text;

#[cfg(test)]
mod tests;

/// summarize the output of SPECTRO
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// only print the vibrational frequency summary
    #[arg(short, long, exclusive = true)]
    vib: bool,

    /// print the output in LaTeX format
    #[arg(short, long, conflicts_with_all = ["json"])]
    tex: bool,

    /// print the output in JSON format
    #[arg(short, long, conflicts_with_all = ["tex"])]
    json: bool,

    infiles: Vec<String>,
}

fn just_vib(summaries: &Vec<Summary>) {
    for sum in summaries {
        println!("Vibrational Frequencies (cm⁻¹):");
        println!("ZPT = {:.1}", sum.zpt);
        let width = 8;
        println!(
            "{:>5}{:>5}{:>width$}{:>width$}{:>width$}",
            "Mode",
            "Symm",
            "Harm",
            "Fund",
            "Corr",
            width = width
        );
        let prec = 1;
        for i in 0..sum.harm.len() {
            println!(
                "{:5}{:>5}{:width$.prec$}{:width$.prec$}{:width$.prec$}",
                i + 1,
                sum.irreps[i],
                sum.harm[i],
                sum.fund[i],
                sum.corr[i],
                width = width,
                prec = prec,
            );
        }
        println!();
    }
}

fn main() {
    let args = Args::parse();
    if args.infiles.is_empty() {
        eprintln!("usage: summarize FILENAME...");
        return;
    }

    let summaries: Vec<_> = args.infiles.iter().map(Summary::new).collect();

    if args.vib {
        just_vib(&summaries);
    } else if args.tex {
        let summaries = format!("{}", Latex(summaries));
        let minus = regex::Regex::new(r"(\s+)-(\d)").unwrap();
        let summaries = minus.replace_all(&summaries, "$1$$-$$$2");
        println!(
            r"\documentclass{{article}}

% for text inside math
\usepackage{{amsmath}}

\begin{{document}}

{}

\end{{document}}
",
            summaries
        );
    } else if args.json {
        println!("\n{}", serde_json::to_string_pretty(&summaries).unwrap());
    } else {
        println!("\n{}", Text(summaries));
    }
}
