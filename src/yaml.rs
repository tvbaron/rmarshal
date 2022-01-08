use std::collections::VecDeque;

const DASHES: &str = "---";
const DOTS: &str = "...";
const NL: &str = "\n";

pub fn read_stream(content: &str) -> Result<VecDeque<String>, ()> {
    let mut content: VecDeque<&str> = content.split("\n").collect();
    let mut docs = VecDeque::new();

    let mut buf = String::new();
    loop {
        match content.pop_front() {
            Some(line) => {
                if line.starts_with(DASHES) {
                    if !buf.is_empty() {
                        let doc = buf.trim().to_owned();
                        if !doc.is_empty() {
                            docs.push_back(doc);
                        }
                        buf.clear();
                    }
                    buf.push_str(line);
                    buf.push_str(NL);
                } else if line == DOTS {
                    buf.push_str(line);
                    buf.push_str(NL);
                    let doc = buf.trim().to_owned();
                    if !doc.is_empty() {
                        docs.push_back(doc);
                    }
                    buf.clear();
                } else {
                    buf.push_str(line);
                    buf.push_str(NL);
                }
            },
            None => {
                if !buf.is_empty() {
                    let doc = buf.trim().to_owned();
                    if !doc.is_empty() {
                        docs.push_back(doc);
                    }
                }
                break;
            }
        }
    } // loop

    Ok(docs)
}
