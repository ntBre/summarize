use summarize::Summary;

fn main() {
    let infile = std::env::args().nth(1)
        .expect("usage: summarize FILENAME");
    println!("{}", Summary::new(&infile));
}
