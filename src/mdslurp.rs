
#[derive(Debug)]
pub enum MarkdownEle {
    Head { txt: String, n: u32 },
    Other { txt: String },
}

impl MarkdownEle {
    pub fn new(line: String, next: Option<&String>) -> MarkdownEle {
        let ch: Vec<char> = line.chars().collect();
        let mut hdrLevel = 0;
        for ch in line.chars() {
            match ch {
                '#' => hdrLevel += 1,
                ' ' => break,
                _ => {
                    hdrLevel = 0;
                    break
                },
            }
        }
        match hdrLevel {
            0 => MarkdownEle::Other { txt: line },
            n => MarkdownEle::Head { txt: line, n: n },
        }
    }
}

