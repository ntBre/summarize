use std::{fmt::Write, fs::read_to_string};
use summarize::Summary;

use crate::{text::Text, latex::Latex};

#[test]
fn text() {
    let summaries = [
        "../testfiles/spectro.out",
        "../testfiles/c2h4.out",
        "../testfiles/allyl.out",
    ];
    let summaries: Vec<_> = summaries.iter().map(Summary::new).collect();
    let mut got = String::new();
    write!(got, "{}", Text(summaries)).unwrap();

    let want = read_to_string("testfiles/want.txt").unwrap();

    if got != want {
        use std::io::Write;
        let mut f = std::fs::File::create("/tmp/got").unwrap();
        write!(f, "{}", got).unwrap();
        panic!(r#"(diff "/tmp/got" "testfiles/want.txt")"#);
    }
}

#[test]
fn latex() {
    let summaries = [
        "../testfiles/spectro.out",
        "../testfiles/c2h4.out",
        "../testfiles/allyl.out",
    ];
    let summaries: Vec<_> = summaries.iter().map(Summary::new).collect();
    let mut got = String::new();
    write!(got, "{}", Latex(summaries)).unwrap();

    let want = read_to_string("testfiles/want.tex").unwrap();

    if got != want {
        use std::io::Write;
        let mut f = std::fs::File::create("/tmp/got").unwrap();
        write!(f, "{}", got).unwrap();
        panic!(r#" (diff "/tmp/got" "summarize-bin/testfiles/want.tex") "#);
    }
}
