use std::cell::RefCell;
use std::iter::Peekable;
use std::iter::Iterator;

use std::iter::FilterMap;
use std::io::Lines;
use std::io::BufReader;
use std::fs::File;

use std::io::Result;

use mdslurp::MarkdownEle;

type LinesIter = FilterMap< Lines<BufReader<File>>,FnMut(Result<String>,) -> Option<String> >;

// iterator howto - thanks Vladimir http://stackoverflow.com/a/27601286

struct MarkdownStream {
    input: RefCell<Peekable<LinesIter>>,
}

impl MarkdownStream {
    pub fn new(mut lines: LinesIter) {
        let lines = lines.peekable();
        MarkdownStream { input: RefCell::new(lines) }
    }
}

impl Iterator for MarkdownStream {
    pub fn next(&mut self) -> Option<MarkdownEle> {
        let lines = self.input.borrow_mut();
        let line = match lines.next() {
            None => return None,
            Some(x) => x,
        };
        let next = lines.peek();
        Some(MarkdownEle::new(line, next))
    }
}
