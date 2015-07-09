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
    let v:Vec<String> = ["Hello", "world"].iter().map(|s| String::from_str(s) + "\n").collect();
    let v_cp = v.to_owned(); // thanks Vladimir http://stackoverflow.com/a/30846725
    let lines: Box<Iterator<Item=String> + 'static> =
        Box::new(v_cp.drain()); // XXX: v.iter() does not make an Iterator<String> ??  Workaround: clone and drain
    // src/mdstream.rs:56:45: 56:63 error: type mismatch resolving `<core::slice::Iter<'_, collections::string::String> as core::iter::Iterator>::Item == collections::string::String`:
    // expected &-ptr,
    // found struct `collections::string::String` [E0271]
    let i = MarkdownStream::new(Box::new(lines));
    assert_eq!(i.next(), Some(MarkdownEle::Other { txt: v[0] }));
}
