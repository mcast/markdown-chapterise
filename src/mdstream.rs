use std::cell::RefCell;
use std::iter::Peekable;
use std::ops::DerefMut;

use std::iter::FilterMap;
use std::io::Lines;
use std::io::BufReader;
use std::fs::File;

use std::io::Result;

use mdslurp::MarkdownEle;

pub type LinesIter   = Iterator<Item = String>;
pub type LinesIoIter = Iterator<Item = Result<String>>;


pub struct MarkdownStream {
    input: Box<Peekable<Box<LinesIter>>>,
}

fn io_unwrap(result: Result<String>) -> Option<String> {
    result.ok()
}

impl MarkdownStream {
    pub fn new_io(lines: Box<LinesIoIter>) -> MarkdownStream {
        let lines: Box<Iterator<Item=String>> = Box::new(lines.filter_map(io_unwrap));
        Self::new(lines)
        // three boxes, hmm
    }
    pub fn new(lines: Box<LinesIter>) -> MarkdownStream {
        let lines = lines.peekable();
        MarkdownStream { input: Box::new(lines) }
    }
}

// iterator howto - thanks http://rustbyexample.com/trait/iter.html

impl Iterator for MarkdownStream {
    type Item = MarkdownEle;
    fn next(&mut self) -> Option<MarkdownEle> {
        let mut lines = self.input.deref_mut();
        let line = match lines.next() {
            None => return None,
            Some(x) => x,
        };
        let next = lines.peek();
        Some(MarkdownEle::new(line, next))
    }
}

#[test]
fn t_vec_others() {
    let v:Vec<&String> = ["Hello", "world"].iter().map(|s| { let o = String::from_str(s) + "\n"; &o }).into_iter().collect::<Vec<&String>>();
    let lines: Box<Iterator<Item=String>> =
        Box::new(v.iter());
    let i = MarkdownStream::new(Box::new(lines));
    assert_eq!(i.next(), Some(MarkdownEle::Other { txt: v[0] }));
}
