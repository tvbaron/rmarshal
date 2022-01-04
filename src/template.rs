use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Clone)]
enum Token {
    Expression(String),
    Statement(String),
    Text(String),
}

#[derive(PartialEq, Eq)]
enum Context {
    Comment,
    Expression,
    Statement,
    StatementLine,
    Text,
}

// Performs a lexical analysis on a given template.
fn tokenize(content: &str) -> Result<VecDeque<Token>, ()> {
    let mut content = VecDeque::from_iter(content.chars());
    let mut tokens = VecDeque::new();

    let mut context = Context::Text;
    let mut buf = String::new();
    loop {
        let curr =
                match content.pop_front() {
                    Some(c) => c,
                    None => {
                        match context {
                            Context::Comment | Context::Expression | Context::Statement => return Err(()),
                            Context::StatementLine => {
                                if !buf.is_empty() {
                                    tokens.push_back(Token::Statement(buf.trim().to_owned()));
                                }
                            },
                            Context::Text => {
                                if !buf.is_empty() {
                                    tokens.push_back(Token::Text(buf.clone()));
                                }
                            },
                        } // context
                        break;
                    },
                };

        match context {
            Context::Comment | Context::Expression | Context::Statement => {
                // Directive.
                if curr == '%' {
                    // '%'
                    match content.pop_front() {
                        Some('>') => {
                            // '%>'
                            // Change of mode -> text.
                            if !buf.is_empty() {
                                match context {
                                    Context::Comment => {
                                        if buf.ends_with("-") {
                                            // Remove whitespaces until the next newline.
                                            let mut cnt = 0;
                                            for (_, c) in content.iter().enumerate() {
                                                if *c == ' ' || *c == '\t' {
                                                    cnt += 1;
                                                    continue;
                                                } else if *c == '\n' {
                                                    cnt += 1;
                                                    break;
                                                } else {
                                                    cnt = 0;
                                                    break;
                                                }
                                            } // for
                                            loop {
                                                if cnt == 0 {
                                                    break;
                                                }

                                                content.pop_front();
                                                cnt -= 1;
                                            } // loop
                                        }
                                    },
                                    Context::Expression => {
                                        tokens.push_back(Token::Expression(buf.trim().to_owned()));
                                    },
                                    Context::Statement => {
                                        if buf.ends_with("-") {
                                            buf.pop();
                                            // Remove whitespaces until the next newline.
                                            let mut cnt = 0;
                                            for (_, c) in content.iter().enumerate() {
                                                if *c == ' ' || *c == '\t' {
                                                    cnt += 1;
                                                    continue;
                                                } else if *c == '\n' {
                                                    cnt += 1;
                                                    break;
                                                } else {
                                                    cnt = 0;
                                                    break;
                                                }
                                            } // for
                                            loop {
                                                if cnt == 0 {
                                                    break;
                                                }

                                                content.pop_front();
                                                cnt -= 1;
                                            } // loop
                                        }
                                        tokens.push_back(Token::Statement(buf.trim().to_owned()));
                                    },
                                    _ => panic!("wtf"),
                                } // match context
                                buf.clear();
                            }
                            context = Context::Text;
                        },
                        Some('%') => {
                            // '%%'
                            // Escape '%'.
                            buf.push('%');
                        },
                        Some(_) => return Err(()),
                        None => return Err(()),
                    } // match content.pop_front()
                } else {
                    buf.push(curr);
                }
            },
            Context::StatementLine => {
                // Directive line.
                if curr == '\n' {
                    // '\n'
                    // Change of mode -> text.
                    tokens.push_back(Token::Statement(buf.trim().to_owned()));
                    buf.clear();
                    context = Context::Text;
                } else {
                    buf.push(curr);
                }
            },
            Context::Text => {
                // Text.
                if curr == '<' {
                    // '<'
                    match content.pop_front() {
                        Some('%') => {
                            // '<%'
                            match content.pop_front() {
                                Some('%') => {
                                    // '<%%'
                                    // Escape '%'.
                                    buf.push('<');
                                    buf.push('%');
                                },
                                Some('#') => {
                                    // '<%#'
                                    // Change of mode -> comment.
                                    if !buf.is_empty() {
                                        tokens.push_back(Token::Text(buf.clone()));
                                        buf.clear();
                                    }
                                    context = Context::Comment;
                                },
                                Some('=') => {
                                    // '<%='
                                    // Change of mode -> expression.
                                    if !buf.is_empty() {
                                        tokens.push_back(Token::Text(buf.clone()));
                                        buf.clear();
                                    }
                                    context = Context::Expression;
                                },
                                Some('-') => {
                                    // '<%-'
                                    // Change of mode -> statement.
                                    if !buf.is_empty() {
                                        let mut cnt = 0;
                                        let mut tmp = buf.clone();
                                        loop {
                                            match tmp.pop() {
                                                Some(c) => {
                                                    if c == ' ' || c == '\t' {
                                                        cnt += 1;
                                                        continue;
                                                    } else if c == '\n' {
                                                        break;
                                                    } else {
                                                        cnt = 0;
                                                        break;
                                                    }
                                                },
                                                None => {
                                                    cnt = 0;
                                                    break;
                                                },
                                            }
                                        } // loop
                                        loop {
                                            if cnt == 0 {
                                                break;
                                            }

                                            buf.pop();
                                            cnt -= 1;
                                        } // loop
                                        tokens.push_back(Token::Text(buf.clone()));
                                        buf.clear();
                                    }
                                    context = Context::Statement;
                                },
                                Some(c) => {
                                    // Change of mode -> statement.
                                    if !buf.is_empty() {
                                        tokens.push_back(Token::Text(buf.clone()));
                                        buf.clear();
                                    }
                                    buf.push(c);
                                    context = Context::Statement;
                                },
                                None => return Err(()),
                            } // match content.pop_front()
                        },
                        Some(c) => {
                            buf.push('<');
                            buf.push(c);
                        },
                        None => {
                            buf.push('<');
                            tokens.push_back(Token::Text(buf.clone()));
                            break;
                        },
                    } // match content.pop_front()
                } else if curr == '%' {
                    // '%'
                    // Check the '%' starts a new line.
                    match buf.chars().last() {
                        Some('\n') => {
                            // '\n%'
                            tokens.push_back(Token::Text(buf.clone()));
                            buf.clear();
                        },
                        Some(_) => {
                            buf.push('%');
                            continue;
                        },
                        None => {
                            // No ongoing text.
                        },
                    } // match buf.chars().last()
                    // Check ahead.
                    match content.pop_front() {
                        Some('\n') => {
                            // '%\n'
                            // Nothing to do.
                        },
                        Some('%') => {
                            // '%%'
                            // Escape '%'.
                            buf.push('%');
                        },
                        Some(c) => {
                            // '%.'
                            buf.push(c);
                            context = Context::StatementLine;
                        },
                        None => break,
                    } // match content.pop_front()
                } else {
                    buf.push(curr);
                }
            },
        } // match context
    } // loop

    Ok(tokens)
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
        let token =
                match tokens.pop_front() {
                    Some(t) => t,
                    None => break,
                };

        match token {
            Token::Expression(e) => {
                #[cfg(feature = "debug")]
                eprintln!("[Expression] '{}'", e);

                res.push_str("table.insert(_sb, ");
                res.push_str(&e);
                res.push_str(")\n");
            },
            Token::Statement(s) => {
                #[cfg(feature = "debug")]
                eprintln!("[Statement] '{}'", s);

                res.push_str(&s);
                res.push_str("\n");
            },
            Token::Text(t) => {
                #[cfg(feature = "debug")]
                eprintln!("[Text] '{}'", t);

                res.push_str("table.insert(_sb, [[\n");
                res.push_str(&t);
                res.push_str("]])\n");
            },
        }
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
