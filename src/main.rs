use summarize::Summary;

fn main() {
    let infile = std::env::args()
        .skip(1)
        .next()
        .expect("usage: summarize FILENAME");
    println!("{}", Summary::new(&infile));
}
