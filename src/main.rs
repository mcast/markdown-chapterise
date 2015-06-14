use std::path::Path;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut args = env::args();
    let prog = args.next().expect("no $0 ?");
    let infile = args.next().expect(& format!("Syntax: {} <input.md> [ <outdir> ]", prog));
    let outdir = match args.next() {
        Some(dir) => dir.as_str().as_ref(),
        None => "./",
    };

    let inpath = Path::new(&infile);
    println!("hopefully {:?} is my file; write to {}", inpath, outdir);

    // thanks A.B. in http://stackoverflow.com/a/25168563
    let infh = BufReader::new(File::open(&inpath).unwrap());
    for line in infh.lines().filter_map(|result| result.ok()) {
        println!("{}", line);
    }
}
