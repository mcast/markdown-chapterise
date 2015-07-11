use std::cell::RefCell;
use std::iter::Peekable;
use std::ops::DerefMut;

use std::iter::FilterMap;
use std::io::Lines;
use std::io::BufReader;
use std::fs::File;

use std::io::Result;

use mdslurp::MarkdownEle;


pub struct MarkdownStream<T: Iterator<Item=String>> {
    input: Box<Peekable<Box<T>>>,
}

fn io_unwrap(result: Result<String>) -> Option<String> {
    result.ok()
}

impl<T: Iterator<Item=String>> MarkdownStream<T> {
//     pub fn new_io<IoT: Iterator<Item=Result<String>>>(lines: Box<IoT>) -> MarkdownStream<T>
    pub fn new_io(lines: Box<Iterator<Item=Result<String>>>) -> MarkdownStream<T>
    {
        let lines: Box<Iterator<Item=String>> =
            Box::new(lines.filter_map(io_unwrap));
        let out = MarkdownStream { input: Box::new(lines.peekable()) };
        out
        // three boxes, hmm
    }
    pub fn new(lines: Box<T>) -> MarkdownStream<T>
    {
        let lines = lines.peekable();
        MarkdownStream { input: Box::new(lines) }
    }
}

// iterator howto - thanks http://rustbyexample.com/trait/iter.html

pub impl<T: Iterator<Item=String>> Iterator for MarkdownStream<T> {
    type Item = MarkdownEle;
    pub fn next(&mut self) -> Option<MarkdownEle> {
        let mut lines = self.input.deref_mut();
        let line = match lines.next() {
            None => return None,
            Some(x) => x,
        };
        let next = lines.peek();
        Some(MarkdownEle::new(line, next))
    }
}

#[cfg(test)]
mod tests {
    use super::MarkdownStream;
    use mdslurp::MarkdownEle;
    use std::slice::Iter;
    use std::iter::Iterator; // in prelude? dnw anyway

    fn stringvec(input: Vec<&str>) -> (Vec<String>, Iter<String>) {
        let out: Vec<String> = input
            .into_iter()
            .map(|s| String::from_str(s) + "\n")
            .collect::<Vec<String>>();
        let v_cp = out.clone();
        (v_cp, out.drain())
    }

    #[test]
    fn t_vec_others() {
        let (v, i) = stringvec(vec!("Hello", "world"));
        let i2: Iterator<Item=String> = i;
        let m = MarkdownStream::new(Box::new(i2));
        assert_eq!(m.next(), Some(MarkdownEle::Other { txt: v[0] }));
    }
}
