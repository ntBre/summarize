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

fn main() {
    let infiles: Vec<_> = std::env::args().skip(1).collect();
    if infiles.is_empty() {
        panic!("usage: summarize FILENAME...");
    }
    let summaries: Vec<_> = infiles.iter().map(Summary::new).collect();

    println!("\n{}", Text(summaries));
}
