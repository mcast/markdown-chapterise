
#[derive(Debug, PartialEq)]
pub enum MarkdownEle {
    Head { txt: String, n: u32 },
    Other { txt: String },
}

impl MarkdownEle {
    pub fn new(line: String, next: Option<&String>) -> MarkdownEle {
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
        // setext (underlined) header detection XXX: a bit fast'n'loose
        match next {
            Some(t) => {
                let mut num_minus = 0;
                let mut num_equ = 0;
                for ch in t.chars() {
                    match ch {
                        '-' => num_minus += 1,
                        '=' => num_equ += 1,
                        _ => {
                            num_minus = 0;
                            num_equ = 0;
                            break
                        },
                    }
                }
                match (hdr_level, num_equ, num_minus) {
                    // setext levels
                    (0, u, 0) => if u > 0 { hdr_level = 1 },
                    (0, 0, u) => if u > 0 { hdr_level = 2 },
                    _ => (),
                }
            },
            None => (),
        };
        match hdr_level {
            0 => MarkdownEle::Other { txt: line },
            n => MarkdownEle::Head { txt: line, n: n },
        }
    }
}


#[test]
fn headcheck() {
    let h = "#### foo";
    let h = h.to_string();
    let junk = "wibble".to_string();
    let out1 = MarkdownEle::new(h.clone(), None);
    let out2 = MarkdownEle::new(h.clone(), Some(&junk));
    let want = MarkdownEle::Head { txt: h, n: 4 };
    assert_eq!(out1, want);
    assert_eq!(out2, want);
}

#[test]
fn othercheck() {
    let ht = "   hello world".to_owned();
    let h1 = "==".to_owned();
    let h2 = "--".to_owned();
    assert_eq!(MarkdownEle::new(ht.clone(), None),
               MarkdownEle::Other { txt: ht.clone() });
    assert_eq!(MarkdownEle::new(ht.clone(), Some(&h1)),
               MarkdownEle::Head { txt: ht.clone(), n: 1 });
    assert_eq!(MarkdownEle::new(ht.clone(), Some(&h2)),
               MarkdownEle::Head { txt: ht.clone(), n: 2 });
}
