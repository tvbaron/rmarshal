use std::collections::VecDeque;
use indexmap::IndexMap;

pub const LONG_OPTION_PREFIX: &str = "--";
pub const LONG_OPTION_PREFIX_LEN: usize = LONG_OPTION_PREFIX.len();

pub const SHORT_OPTION_PREFIX: &str = "-";
pub const SHORT_OPTION_PREFIX_LEN: usize = SHORT_OPTION_PREFIX.len();

// Splits a key-value pair with an optional value.
fn parse_pair(pair: &str) -> Result<(String, Option<String>), ()> {
    match pair.find("=") {
        Some(idx) => {
            let key =
                    match pair.get(0..idx) {
                        Some(v) => v,
                        None => return Err(()),
                    };
            let val =
                    match pair.get(idx + 1..) {
                        Some(v) => v,
                        None => return Err(()),
                    };

            Ok((key.to_owned(), Some(val.to_owned())))
        },
        None => Ok((pair.to_owned(), None)),
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct FlaggedOption {
    // Whether this one is a short option, i.e. only one "-" prefix.
    pub short: bool,
    // The actual option.
    pub option: String,
    // The optional value.
    pub value: Option<String>,
    // The options qualifiers.
    pub qualifiers: Option<IndexMap<String, Option<String>>>,
}

impl FlaggedOption {
    pub fn from_str(opt: &str) -> Result<FlaggedOption, ()> {
        if opt.starts_with(LONG_OPTION_PREFIX) {
            // Long option.
            let rem =
                    match opt.get(LONG_OPTION_PREFIX_LEN..) {
                        Some(c) => c,
                        None => return Err(()),
                    };
            let mut comps = rem.split(":").collect::<VecDeque<&str>>();
            let (option, value) =
                    match comps.pop_front() {
                        Some(c) => {
                            parse_pair(c)?
                        },
                        None => return Err(()),
                    };
            if comps.is_empty() {
                // No qualifiers.

                Ok(FlaggedOption {
                    short: false,
                    option,
                    value,
                    qualifiers: None,
                })
            } else {
                // With qualifiers.
                let mut qualifiers = IndexMap::new();
                loop {
                    match comps.pop_front() {
                        Some(c) => {
                            let (key, val) = parse_pair(c)?;
                            qualifiers.insert(key, val);
                        },
                        None => break,
                    }
                } // loop

                Ok(FlaggedOption {
                    short: false,
                    option,
                    value,
                    qualifiers: Some(qualifiers),
                })
            }
        } else if opt.starts_with(SHORT_OPTION_PREFIX) {
            // Short option.
            let option =
                    match opt.get(SHORT_OPTION_PREFIX_LEN..SHORT_OPTION_PREFIX_LEN + 1) {
                        Some(c) => c,
                        None => return Err(()),
                    };
            if opt.len() > SHORT_OPTION_PREFIX_LEN + 1 {
                let value =
                        match opt.get(SHORT_OPTION_PREFIX_LEN + 1..) {
                            Some(c) => c,
                            None => return Err(()),
                        };

                Ok(FlaggedOption {
                    short: true,
                    option: option.to_owned(),
                    value: Some(value.to_owned()),
                    qualifiers: None,
                })
            } else {
                Ok(FlaggedOption {
                    short: true,
                    option: option.to_owned(),
                    value: None,
                    qualifiers: None,
                })
            }
        } else {
            Err(())
        }
    }
}


// --stream=endless:limit=5:yo=Thomas

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_pair {
        use super::*;

        #[test]
        fn it_parse() {
            let (key, val) = parse_pair("foo").unwrap();
            assert_eq!(key, "foo".to_owned());
            assert_eq!(val, None);
        }

        #[test]
        fn it_parse_value() {
            let (key, val) = parse_pair("foo=bar").unwrap();
            assert_eq!(key, "foo".to_owned());
            assert_eq!(val, Some("bar".to_owned()));
        }
    }

    mod flagged_option {
        use super::*;

        #[test]
        fn it_does_not_parse() {
            let res = FlaggedOption::from_str("foo");
            assert_eq!(res, Err(()));
        }

        #[test]
        fn it_parse_long() {
            let res = FlaggedOption::from_str("--foo").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, None);
            assert_eq!(res.qualifiers, None);
        }

        #[test]
        fn it_parse_long_value() {
            let res = FlaggedOption::from_str("--foo=bar").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, Some("bar".to_owned()));
            assert_eq!(res.qualifiers, None);
        }

        #[test]
        fn it_parse_long_qualifier() {
            let res = FlaggedOption::from_str("--foo:alfa").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, None);
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 1);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, None);
        }

        #[test]
        fn it_parse_long_qualifier_value() {
            let res = FlaggedOption::from_str("--foo:alfa=1").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, None);
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 1);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, Some("1".to_owned()));
        }

        #[test]
        fn it_parse_long_qualifiers() {
            let res = FlaggedOption::from_str("--foo:alfa:bravo").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, None);
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 2);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, None);
            let bravo_val = qualifiers.get("bravo").unwrap().clone();
            assert_eq!(bravo_val, None);
        }

        #[test]
        fn it_parse_long_qualifiers_value() {
            let res = FlaggedOption::from_str("--foo:alfa=1:bravo").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, None);
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 2);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, Some("1".to_owned()));
            let bravo_val = qualifiers.get("bravo").unwrap().clone();
            assert_eq!(bravo_val, None);
        }

        #[test]
        fn it_parse_long_qualifiers_value_value() {
            let res = FlaggedOption::from_str("--foo:alfa=1:bravo=2").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, None);
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 2);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, Some("1".to_owned()));
            let bravo_val = qualifiers.get("bravo").unwrap().clone();
            assert_eq!(bravo_val, Some("2".to_owned()));
        }

        #[test]
        fn it_parse_long_value_qualifier() {
            let res = FlaggedOption::from_str("--foo=bar:alfa").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, Some("bar".to_owned()));
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 1);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, None);
        }

        #[test]
        fn it_parse_long_value_qualifier_value() {
            let res = FlaggedOption::from_str("--foo=bar:alfa=1").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, Some("bar".to_owned()));
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 1);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, Some("1".to_owned()));
        }

        #[test]
        fn it_parse_long_value_qualifiers() {
            let res = FlaggedOption::from_str("--foo=bar:alfa:bravo").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, Some("bar".to_owned()));
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 2);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, None);
            let bravo_val = qualifiers.get("bravo").unwrap().clone();
            assert_eq!(bravo_val, None);
        }

        #[test]
        fn it_parse_long_value_qualifiers_value() {
            let res = FlaggedOption::from_str("--foo=bar:alfa=1:bravo").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, Some("bar".to_owned()));
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 2);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, Some("1".to_owned()));
            let bravo_val = qualifiers.get("bravo").unwrap().clone();
            assert_eq!(bravo_val, None);
        }

        #[test]
        fn it_parse_long_value_qualifiers_value_value() {
            let res = FlaggedOption::from_str("--foo=bar:alfa=1:bravo=2").unwrap();
            assert_eq!(res.short, false);
            assert_eq!(res.option, "foo".to_owned());
            assert_eq!(res.value, Some("bar".to_owned()));
            let qualifiers = res.qualifiers.unwrap();
            assert_eq!(qualifiers.len(), 2);
            let alfa_val = qualifiers.get("alfa").unwrap().clone();
            assert_eq!(alfa_val, Some("1".to_owned()));
            let bravo_val = qualifiers.get("bravo").unwrap().clone();
            assert_eq!(bravo_val, Some("2".to_owned()));
        }

        #[test]
        fn it_parse_short() {
            let res = FlaggedOption::from_str("-f").unwrap();
            assert_eq!(res.short, true);
            assert_eq!(res.option, "f".to_owned());
            assert_eq!(res.value, None);
            assert_eq!(res.qualifiers, None);
        }

        #[test]
        fn it_parse_short_value() {
            let res = FlaggedOption::from_str("-fbar").unwrap();
            assert_eq!(res.short, true);
            assert_eq!(res.option, "f".to_owned());
            assert_eq!(res.value, Some("bar".to_owned()));
            assert_eq!(res.qualifiers, None);
        }
    }
}
