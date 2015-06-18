
#[derive(Debug)]
pub enum MarkdownEle {
    Head { txt: String, n: u32 },
    Other { txt: String },
}

impl MarkdownEle {
    pub fn new(line: String, next: Option<&String>) -> MarkdownEle {
        let ch: Vec<char> = line.chars().collect();
        let mut hdr_level = 0;
        for ch in line.chars() {
            match ch {
                '#' => hdr_level += 1,
                ' ' => break,
                _ => {
                    hdr_level = 0;
                    break
                },
            }
        }
        match hdr_level {
            0 => MarkdownEle::Other { txt: line },
            n => MarkdownEle::Head { txt: line, n: n },
        }
    }
}

