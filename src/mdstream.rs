
type MarkdownStream<F: FnMut(Result<String>,) -> Option<String>> =
    Peekable<FilterMap<Lines<BufReader<File>>, F>>;

fn ele_iter(lines: &mut MarkdownStream) -> Option<MarkdownEle> {
    let line = match lines.next() {
        None => return None,
        Some(x) => x,
    };
    let next = lines.peek();
    Some(MarkdownEle::new(line, next))
}



// thanks Vladimir http://stackoverflow.com/a/27601286 :

struct MarkdownIn<'a> {
    MarkdownStream &'a input,
}

impl<'a> Iterator<MarkdownEle> for MarkdownIn<'a> {
    pub fn new(in) {
        MarkdownStream { input: in }
    }

    fn next(&mut self) -> Option<MarkdownEle> {
        
    }
}
