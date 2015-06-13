use std::path::Path;
use std::env;

fn main() {
    let mut args = env::args();
    let prog = args.next().expect("no $0 ?");
    let infile = args.next().expect(& format!("Syntax: {} <input.md>", prog));
    let inpath = Path::new(&infile);
    println!("hopefully {:?} is my file", inpath)
}
