use std::path::Path;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::io::Result;

use std::iter::{Peekable, FilterMap};
use std::io::Lines;

mod mdslurp;
use mdslurp::MarkdownEle;

mod mdout;
use mdout::MarkdownOut;

mod mdstream;
use mdstream::MarkdownStream;


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
    let lines = input.lines().filter_map(|result| result.ok());
    let mut output = showerror("create", MarkdownOut::new(outdir, "prelude"));
    for ele in MarkdownStream::new(lines) {
        let t = match ele {
            MarkdownEle::Other { txt } => txt,
            MarkdownEle::Head { txt, n } => {
                if n <= split_depth {
                    showerror("close", output.close());
                    let res = output.next("XXX,chaptername");
                    output = showerror("create next", res)
                }
                txt
            },
        };
        showerror("append", output.append(t));
    }
}

fn showerror<T>(operation: &str, r: Result<T>) -> T {
    match r {
        Err(err) => panic!("{}: {}", operation, err),
        Ok(ret) => ret,
    }
}

#[test]
fn t_showerror_ok() {
    let obj = "bar";
    let res : std::io::Result<_> = std::result::Result::Ok(obj);
    assert_eq!(showerror("foo", res), obj);
}

#[test]
#[should_panic]
fn t_showerror_err() {
    let obj = "bar";
    let err = std::io::Error::new(std::io::ErrorKind::Other, "error text");
    let res : std::io::Result<u32> = std::result::Result::Err(err);
    let _ = showerror("bar", res); // XXX: can't demonstrate useful error message output
}
