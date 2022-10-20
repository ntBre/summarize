use summarize::Summary;

fn main() {
    let infiles: Vec<_> = std::env::args().skip(1).collect();
    if infiles.is_empty() {
        panic!("usage: summarize FILENAME...");
    }
    for infile in infiles {
        println!("{}", Summary::new(&infile));
    }
}
