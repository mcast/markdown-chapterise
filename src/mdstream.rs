use std::cell::RefCell;
use std::iter::Peekable;

use std::iter::FilterMap;
use std::io::Lines;
use std::io::BufReader;
use std::fs::File;

use std::io::Result;

use mdslurp::MarkdownEle;

type LinesIoIter = Lines<BufReader<File>>;

// iterator howto - thanks Vladimir http://stackoverflow.com/a/27601286

struct MarkdownStream {
    input: RefCell<Peekable<FilterMap<LinesIoIter,FnMut()>>>,
}

fn io_unwrap<T>(result: Result<T>) -> Option<T> {
    result.ok()
}

impl MarkdownStream {
    pub fn new(mut lines: LinesIoIter) {
        let lines = lines.filter_map(io_unwrap).peekable();
        MarkdownStream { input: RefCell::new(lines) }
    }
}

impl Iterator for MarkdownStream {
    type Item = MarkdownEle;
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
