use std::path::Path;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut args = env::args();
    let prog = args.next().expect("no $0 ?");
    let infile = args.next().expect(& format!("Syntax: {} <input.md> [ <outdir> ]", prog));
    let dir = args.next();
    let outdir = match dir {
        Some(ref dir) => dir.as_ref(),
        None => "./",
    };

    let inpath = Path::new(&infile);
    let outpath = Path::new(&outdir);

    // thanks A.B. in http://stackoverflow.com/a/25168563
    let infh = BufReader::new(File::open(&inpath).unwrap());
    do_split(infh, &outpath, 2);
}

fn do_split(input: BufReader<File>, outdir: &Path, split_depth: u32) {
    println!("write to {:?}", outdir);

    for line in input.lines().filter_map(|result| result.ok()) {
        println!("{}",  line);
    }
}
