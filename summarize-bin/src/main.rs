use clap::Parser;

use summarize::Summary;

use crate::text::Text;

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

macro_rules! write_dist_consts {
    ($w:ident, $iter: expr, $struct:ident,
     $($field:ident => $name:expr$(,)?),*) => {
	$(
	    write!($w, "{:<8}", $name)?;
	    for sum in $iter {
		if let Some(d) = sum.$struct.$field {
		    write!($w, "{:18.10}", d)?;
		} else {
		    write!($w, "{:18.10}", "")?;
		}
	    }
	    writeln!($w)?;
	)*
    };
}

mod text;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// only print the vibrational frequency summary
    #[arg(short, long)]
    vib: bool,

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
        panic!("usage: summarize FILENAME...");
    }

    let summaries: Vec<_> = args.infiles.iter().map(Summary::new).collect();

    if args.vib {
        just_vib(&summaries);
    } else {
        println!("\n{}", Text(summaries));
    }
}
