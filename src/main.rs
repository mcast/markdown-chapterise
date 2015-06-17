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

#[derive(Debug)]
enum MarkdownEle {
    Head { txt: String, n: u32 },
    Other { txt: String },
}

impl MarkdownEle {
    fn new(line: String, next: Option<&String>) -> MarkdownEle {
        let ch: Vec<char> = line.chars().collect();
        MarkdownEle::Other { txt: line }
    }
}


fn do_split(input: BufReader<File>, outdir: &Path, split_depth: u32) {
    println!("write to {:?}", outdir);

    let lines = input.lines().filter_map(|result| result.ok());
    let mut lines = lines.peekable();
    loop {
        let line = match lines.next() {
            None => break,
            Some(x) => x,
        };
        let next = lines.peek();
        let ele = MarkdownEle::new(line, next);
        println!("{:?}", ele);
    }
}
