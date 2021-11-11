use std::collections::LinkedList;

#[derive(PartialEq, Eq, Clone)]
enum Token {
    Expression(String),
    NewLine,
    Statement(String, bool, bool), // (statement: String, trim_before: bool, trim_after: bool).
    Text(String),
}

impl Token {
    fn is_newline(&self) -> bool {
        *self == Token::NewLine
    }

    fn is_statement_trim_before(&self) -> bool {
        if let Token::Statement(_, trim_before, _) = self {
            *trim_before == true
        } else {
            false
        }
    }

    fn is_statement_trim_after(&self) -> bool {
        if let Token::Statement(_, _, trim_after) = self {
            *trim_after == true
        } else {
            false
        }
    }

    fn is_blank_text(&self) -> bool {
        if let Token::Text(t) = self {
            for c in t.chars() {
                if c == ' ' || c == '\t' {
                    continue;
                }

                return false;
            } // for

            true
        } else {
            false
        }
    }
}

// Performs a lexical analysis on a given template.
fn tokenize(content: &str) -> Result<LinkedList<Token>, ()> {
    let mut content = content.to_owned();
    let mut tokens = LinkedList::new();

    // let mut loop_cnt = 0;
    loop {
        if content.is_empty() {
            break;
        }

        // loop_cnt += 1;
        // println!("[{}]->{}<-", loop_cnt, content);

        let mut newline_idx =
                match content.find("\n") {
                    Some(i) => i as i64,
                    None => -1,
                };
        let mut tag_idx =
                match content.find("<%") {
                    Some(i) => i as i64,
                    None => -1,
                };

        if newline_idx < 0 && tag_idx < 0 {
            // This is the end of the template.
            tokens.push_back(Token::Text(content));
            break;
        }

        assert_ne!(newline_idx, tag_idx);

        // Check what is coming first between the newline and the tag.
        if newline_idx >= 0 && tag_idx >= 0 {
            // There are both a newline and a tag ahead.
            if newline_idx < tag_idx {
                // The newline comes before the tag.
                tag_idx = -1;
            } else {
                // The tag comes before the newline.
                newline_idx = -1;
            }
        }

        if newline_idx >= 0 {
            // Process a line without tag.
            if newline_idx > 0 {
                let i = newline_idx as usize;
                let c: String = content.drain(..i).collect();
                tokens.push_back(Token::Text(c));
            }
            content.remove(0);
            tokens.push_back(Token::NewLine);
        }

        else if tag_idx >= 0 {
            // Process a tag.
            if tag_idx > 0 {
                // There is some text before the tag.
                let i = tag_idx as usize;
                let c: String = content.drain(..i).collect();
                tokens.push_back(Token::Text(c));
            }
            if let Some(end_idx) = content.find("%>") {
                if content.starts_with("<%=") {
                    // Deal with an expression.
                    let c: String = content.drain(3..end_idx).collect();
                    tokens.push_back(Token::Expression(c.trim().to_owned()));
                    content.drain(..5);
                } else {
                    // Deal with a statement.
                    let mut c: String = content.drain(2..end_idx).collect();
                    let mut trim_before = false;
                    let mut trim_after = false;
                    if c.starts_with("-") {
                        trim_before = true;
                        c.remove(0);
                    }
                    if c.ends_with("-") {
                        trim_after = true;
                        c.remove(c.len() - 1);
                    }
                    tokens.push_back(Token::Statement(c.trim().to_owned(), trim_before, trim_after));
                    content.drain(..4);
                }
            } else {
                // A tag must be closed.
                return Err(());
            }
        }

        else {
            panic!("wtf");
        }
    } // loop

    Ok(tokens)
}

// Collects the next line.
fn collect_line(tokens: &mut LinkedList<Token>) -> Vec<Token> {
    let mut res = Vec::new();
    let mut newline = false;

    // (1of3) Collect the next line.
    loop {
        if let Some(token) = tokens.pop_front() {
            if token.is_newline() {
                newline = true;
                break;
            }

            res.push(token.clone());
        } else {
            break;
        }
    } // loop

    // (2of3) Trim blank text before/after statements if necessary.
    let mut idx = 0;
    loop {
        let rem = res.len() - idx;
        if rem <= 1 {
            break;
        }

        if res[idx].is_blank_text() && res[idx + 1].is_statement_trim_before() {
            res.remove(0);
            continue;
        } else if res[idx].is_statement_trim_after() && res[idx + 1].is_blank_text() {
            res.remove(1);
        }

        idx += 1;
    } // loop

    // (3of3) Add a newline if necessary.
    if newline && (res.len() != 1 || !res[0].is_statement_trim_after()) {
        res.push(Token::NewLine);
    }

    res
}

// Converts a given template into lua code.
fn parse_template(content: &str) -> Result<String, ()> {
    let mut tokens =
            match tokenize(content) {
                Ok(t) => t,
                Err(_) => return Err(()),
            };

    let mut res = String::new();
    res.push_str("local _sb = {}\n");

    loop {
        if tokens.is_empty() {
            break;
        }

        let line = collect_line(&mut tokens);
        for token in &line {
            match token {
                Token::Expression(e) => {
                    res.push_str("table.insert(_sb, ");
                    res.push_str(e);
                    res.push_str(")\n");
                },
                Token::NewLine => {
                    res.push_str("table.insert(_sb, [[\n");
                    res.push_str("\n");
                    res.push_str("]])\n");
                },
                Token::Statement(s, _, _) => {
                    res.push_str(s);
                    res.push_str("\n");
                },
                Token::Text(t) => {
                    res.push_str("table.insert(_sb, [[");
                    res.push_str(t);
                    res.push_str("]])\n");
                },
            }
        } // for
    } // loop
    res.push_str("ctx:set_output(_sb)\n");

    Ok(res)
}

pub struct Template {
    pub content: String,
}

impl Template {
    pub fn for_path(path: &str) -> Self {
        let template_content =
                match std::fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(e) => panic!("{}", e),
                };

        Template {
            content: parse_template(&template_content).unwrap(),
        }
    }
}
