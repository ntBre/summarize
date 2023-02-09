use std::{
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
    str::FromStr,
};

use clap::Parser;

use summarize::{Recompute, Summary, SYMM_EPS};

use crate::{csv::Csv, latex::Latex, org::Org, text::Text};

mod tui;

/// macro for generating max_* methods on `Text`, inspired by this
/// https://github.com/jonhoo/fantoccini/pull/186#discussion_r990712599
/// suggestion and by the actual implementation
/// https://github.com/jonhoo/fantoccini/pull/186/commits/8035a970e
macro_rules! max_fields {
    ($($field:ident => $name:ident$(,)?),*) => {
	$(
	    fn $name(&self) -> usize {
		self.summaries.iter().map(|sum| sum.$field.len()).max().unwrap()
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
    const UNITS: [(f64, Unit); 7] = [
        //
        (1e12, Unit::uHz),
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
	    let vals: Vec<_> = $iter.into_iter().map(|sum| sum.$struct.$field).collect();
	    if vals.iter().any(std::option::Option::is_some) {
		let (vals, unit) = crate::find_units(vals);
		write!($w, "{}{:<13}{}{:<8}{}", $iter.pre(), $name, $iter.sep(),
		       $iter.format_dist_unit(unit), $iter.sep())?;
		for (i, v) in vals.iter().enumerate() {
		    if let Some(d) = v {
			write!($w, "{:10.3}", d)?;
		    } else {
			write!($w, "{:10.3}", "")?;
		    }
		    write!($w, "{}", $iter.end(i < nsum-1))?;
		}

		writeln!($w)?;
	    }
	)*
    };
}

mod csv;
mod format;
mod latex;
mod org;
mod text;

#[cfg(test)]
mod tests;

/// summarize the output of SPECTRO
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// only print the vibrational frequency summary
    #[arg(short, long, conflicts_with_all = ["tex", "json", "csv", "org"])]
    vib: bool,

    /// print the output in LaTeX format
    #[arg(short, long, conflicts_with_all = ["json", "csv", "org"])]
    tex: bool,

    /// print the output in JSON format
    #[arg(short, long, conflicts_with_all = ["tex", "csv", "org"])]
    json: bool,

    /// print the output in CSV format
    #[arg(short, long, conflicts_with_all = ["tex", "json", "org"])]
    csv: bool,

    /// print the output in org format for Emacs
    #[arg(short, long, conflicts_with_all = ["tex", "json", "csv"])]
    org: bool,

    /// if reading a rust spectro output file (file ending with .json),
    /// recompute the point group and irreps from the geometry and LXM matrix
    #[arg(short, long, default_value_t = false)]
    recompute_irreps: bool,

    #[arg(short, long, default_value_t = SYMM_EPS)]
    eps_irreps: f64,

    /// launch the tui application to diff two qffs
    #[arg(short, long, default_value_t = false)]
    diff: bool,

    /// load plain text data from FILE as the last argument
    #[arg(short, long, default_value = None)]
    plain: Option<String>,

    /// provide a comma-separated list of names to use for the summaries instead
    /// of Mol. 1, Mol. 2, etc.
    #[arg(short, long, default_value = None)]
    names: Option<String>,

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

fn load_plain<P>(p: P) -> Summary
where
    P: AsRef<Path>,
{
    let f = std::fs::File::open(p).unwrap();
    let lines = BufReader::new(f).lines();
    let mut irreps = Vec::new();
    let mut harm = Vec::new();
    let mut corr = Vec::new();
    for line in lines.flatten() {
        let sp: Vec<_> = line.split_ascii_whitespace().collect();
        if sp.len() == 3 {
            // line like SYMM HARM FUND
            irreps.push(symm::Irrep::from_str(sp[0]).unwrap());
            harm.push(sp[1].parse().unwrap());
            corr.push(sp[2].parse().unwrap());
        }
    }
    Summary {
        harm,
        fund: corr.clone(),
        corr,
        irreps,
        ..Default::default()
    }
}

fn main() {
    let args = Args::parse();
    if args.infiles.is_empty() && args.plain.is_none() {
        eprintln!("usage: summarize FILENAME...");
        return;
    }

    let recompute = if args.recompute_irreps {
        Recompute::Yes(args.eps_irreps)
    } else {
        Recompute::No
    };

    let mut summaries: Vec<_> = args
        .infiles
        .iter()
        .map(|f| Summary::new(f, recompute))
        .collect();

    if let Some(p) = args.plain {
        summaries.push(load_plain(p));
    }

    let names = if let Some(names) = args.names {
        names.split(',').map(|s| s.trim().to_owned()).collect()
    } else {
        default_names(&summaries)
    };

    if names.len() != summaries.len() {
        eprintln!(
            "{} names provided for {} summaries",
            names.len(),
            summaries.len()
        );
        exit(1);
    }

    if args.diff {
        if summaries.len() != 2 {
            eprintln!("usage: summarize -d FILE1 FILE2");
            return;
        }

        tui::run_tui(summaries, names).unwrap();
        return;
    }

    if args.vib {
        just_vib(&summaries);
    } else if args.tex {
        let summaries = format!("{}", Latex { summaries, names });
        let minus = regex::Regex::new(r"(\s+)-(\d)").unwrap();
        let summaries = minus.replace_all(&summaries, "$1$$-$$$2");
        println!(
            r"\documentclass{{article}}

% for text inside math
\usepackage{{amsmath}}

\begin{{document}}

{summaries}

\end{{document}}
"
        );
    } else if args.json {
        println!("\n{}", serde_json::to_string_pretty(&summaries).unwrap());
    } else if args.csv {
        println!("\n{}", Csv { summaries, names });
    } else if args.org {
        println!("\n{}", Org { summaries, names });
    } else {
        println!("\n{}", Text { summaries, names });
    }
}

fn default_names(summaries: &Vec<Summary>) -> Vec<String> {
    let mut names = Vec::new();
    for i in 0..summaries.len() {
        names.push(format!("Mol. {}", i + 1));
    }
    names
}
