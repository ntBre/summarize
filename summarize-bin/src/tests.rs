use std::{fmt::Write, fs::read_to_string};
use summarize::{Recompute, Summary};

use crate::{default_names, latex::Latex, text::Text};

#[test]
fn text() {
    let summaries = [
        "../testfiles/spectro.out",
        "../testfiles/c2h4.out",
        "../testfiles/allyl.out",
    ];
    let summaries: Vec<_> = summaries
        .iter()
        .map(|s| Summary::new(s, Recompute::No))
        .collect();
    let mut got = String::new();
    let names = default_names(&summaries);
    write!(got, "{}", Text { summaries, names }).unwrap();

    let want = read_to_string("testfiles/want.txt").unwrap();

    if got != want {
        use std::io::Write;
        let mut f = std::fs::File::create("/tmp/got").unwrap();
        write!(f, "{got}").unwrap();
        panic!(r#" (diff "/tmp/got" "summarize-bin/testfiles/want.txt") "#);
    }
}

#[test]
fn latex() {
    let summaries = [
        "../testfiles/spectro.out",
        "../testfiles/c2h4.out",
        "../testfiles/allyl.out",
    ];
    let summaries: Vec<_> = summaries
        .iter()
        .map(|s| Summary::new(s, Recompute::No))
        .collect();
    let mut got = String::new();
    let names = default_names(&summaries);
    write!(got, "{}", Latex { summaries, names }).unwrap();

    let want = read_to_string("testfiles/want.tex").unwrap();

    if got != want {
        use std::io::Write;
        let mut f = std::fs::File::create("/tmp/got").unwrap();
        write!(f, "{got}").unwrap();
        panic!(r#" (diff "/tmp/got" "summarize-bin/testfiles/want.tex") "#);
    }
}
