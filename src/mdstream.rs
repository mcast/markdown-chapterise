use std::cell::RefCell;
use std::iter::Peekable;
use std::ops::DerefMut;

use std::iter::FilterMap;
use std::io::Lines;
use std::io::BufReader;
use std::fs::File;

use std::io::Result;

use mdslurp::MarkdownEle;


pub struct MarkdownStream {
    input: Box<Peekable<Box<Iterator<Item=String>>>>,
}

fn io_unwrap(result: Result<String>) -> Option<String> {
    result.ok()
}

impl MarkdownStream {
//     pub fn new_io<IoT: Iterator<Item=Result<String>>>(lines: Box<IoT>) -> MarkdownStream
    pub fn new_io(lines: Box<Iterator<Item=Result<String>>>) -> MarkdownStream
    {
        let lines: Box<Iterator<Item=String>> =
            Box::new(lines.filter_map(io_unwrap));
        let out = MarkdownStream { input: Box::new(lines.peekable()) };
        out
        // three boxes, hmm
    }
    pub fn new(lines: Box<Iterator<Item=String>>) -> MarkdownStream
    {
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

#[cfg(test)]
mod tests {
    use super::MarkdownStream;
    use mdslurp::MarkdownEle;
    use std::vec::IntoIter;
    use std::iter::Iterator; // in prelude? dnw anyway

    fn stringvec(input: Vec<&str>) -> (Vec<String>, IntoIter<String>) {
        let out: Vec<String> = input
            .into_iter()
            .map(|s| s.to_string() + "\n")
            .collect::<Vec<String>>();
        let v_cp = out.clone();
        (v_cp, out.into_iter())
    }

    #[test]
    fn t_vec_others() {
        let (v, i) = stringvec(vec!("Hello", "world"));
        let mut m = MarkdownStream::new(Box::new(i));
        assert_eq!(m.next(), Some(MarkdownEle::Other { txt: v[0].to_owned() }));
    }
}
