extern crate indexmap;
extern crate lazy_static;
extern crate regex;
extern crate rlua;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

mod help;

mod util;
use crate::util::{
    LONG_OPTION_PREFIX,
    LONG_OPTION_PREFIX_LEN,
    SHORT_OPTION_PREFIX,
    SHORT_OPTION_PREFIX_LEN,
    FlaggedOption,
};

mod unit;
use crate::unit::{
    FileFormat,
    DocumentHint,
    UnitDocument,
    UnitFile,
    UnitCommand,
    Unit,
};

mod value;
use crate::value::Value;

mod command;
mod template;
mod yaml;

const STDIO_PLACEHOLDER: &str = "-";

const HELP_CMD: &str = "--help";
const VERSION_CMD: &str = "--version";

const VERSION: &str = env!("CARGO_PKG_VERSION");

// Reads the content of a file.
fn read_content(path: &String) -> Result<String, std::io::Error> {
    if path == STDIO_PLACEHOLDER {
        // Read from STDIN instead.
        let mut sb = String::new();
        let stdin = std::io::stdin();
        loop {
            match stdin.read_line(&mut sb) {
                Ok(0) => break,
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        } // loop

        Ok(sb)
    } else {
        let content =
                match std::fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(e) => return Err(e),
                };

        Ok(content)
    }
}

fn create_lua_value(content: &str) -> Result<Value, ()> {
    let mut lua_content = String::new();
    lua_content.push_str("ctx:set_output(");
    lua_content.push_str(content);
    lua_content.push_str(")\n");
    let lua = rlua::Lua::new();
    lua.context(|lua_ctx| {
        match lua_ctx.load(command::LUA_PRELUDE).exec() {
            Ok(_) => {},
            Err(_) => return Err(()),
        }

        let globals = lua_ctx.globals();

        let ctx: rlua::Table =
                match globals.get("ctx") {
                    Ok(v) => v,
                    Err(_) => return Err(()),
                };

        match lua_ctx.load(&lua_content).exec() {
            Ok(_) => {},
            Err(_) => return Err(()),
        }

        let outputs: rlua::Table =
                match ctx.get("outputs") {
                    Ok(v) => v,
                    Err(_) => return Err(()),
                };

        match value::from_lua_table(outputs) {
            Value::Array(a) => Ok(a[0].clone()),
            _ => return Err(()),
        }
    })
}

// Creates a document.
fn create_document(hint: DocumentHint, content: &str) -> Result<Value, ()> {
    match hint {
        DocumentHint::Any => {
            lazy_static! {
                static ref INTEGER_RE: Regex = Regex::new("^[+-]?[0-9]+$").unwrap();
                static ref FLOAT_RE: Regex = Regex::new("^[+-]?([0-9]+([.][0-9]*)?|[.][0-9]+)$").unwrap();
            }
            let lc_val = content.to_lowercase();
            if content == "~" {
                Ok(Value::Nil)
            } else if lc_val == "false" || lc_val == "off" {
                Ok(Value::Boolean(false))
            } else if lc_val == "true" || lc_val == "on" {
                Ok(Value::Boolean(true))
            } else if INTEGER_RE.is_match(content) {
                let val =
                        match content.parse::<i64>() {
                            Ok(v) => v,
                            Err(_) => return Err(()),
                        };

                Ok(Value::Integer(val))
            } else if FLOAT_RE.is_match(content) {
                let val =
                        match content.parse::<f64>() {
                            Ok(v) => v,
                            Err(_) => return Err(()),
                        };

                Ok(Value::Float(val))
            } else {
                Ok(Value::String(content.to_owned()))
            }
        },
        DocumentHint::Nil => {
            if content == "~" {
                Ok(Value::Nil)
            } else {
                Err(())
            }
        },
        DocumentHint::Boolean => {
            let lc_val = content.to_lowercase();
            match lc_val.as_str() {
                "false" | "off" => Ok(Value::Boolean(false)),
                "true" | "on" => Ok(Value::Boolean(true)),
                _ => Err(()),
            }
        },
        DocumentHint::Integer => {
            let val =
                    match content.parse::<i64>() {
                        Ok(v) => v,
                        Err(_) => return Err(()),
                    };

            Ok(Value::Integer(val))
        },
        DocumentHint::Float => {
            let val =
                    match content.parse::<f64>() {
                        Ok(v) => v,
                        Err(_) => return Err(()),
                    };

            Ok(Value::Float(val))
        },
        DocumentHint::String => Ok(Value::String(content.to_owned())),
        DocumentHint::Lua => {
            let value =
                    match create_lua_value(content) {
                        Ok(v) => v,
                        Err(_) => panic!("cannot create lua value"),
                    };

            Ok(value)
        },
    }
}

/**
 * Exit codes:
 * - INTERNAL_ERROR(1)
 * - WRONG_PARAMETER(10)
 * - UNKNOWN_FILE_FORMAT(11)
 * - NO_INPUT(20)
 * - WRONG_INPUT(21)
 * - NO_OUTPUT(30)
 * - WRONG_OUTPUT(31)
 */
fn main() {
    // (1ofx) Parse arguments.
    let mut units: VecDeque<Unit> = VecDeque::new();
    let mut args: VecDeque<String> = std::env::args().skip(1).collect();
    // Handle '--version' and '--help [TOPIC]'.
    match args.len() {
        0 => {
            eprintln!("wrong parameter");
            std::process::exit(10);
        },
        1 => {
            match args.front().unwrap().as_str() {
                HELP_CMD => {
                    println!("{}", help::GLOBAL_HELP);
                    std::process::exit(0);
                },
                VERSION_CMD => {
                    println!("rmarshal {}", VERSION);
                    std::process::exit(0);
                },
                _ => {},
            }
        },
        2 => {
            match args.front().unwrap().as_str() {
                HELP_CMD => {
                    args.pop_front();
                    match args.pop_front().unwrap().as_str() {
                        "check" => println!("{}", help::CHECK_HELP),
                        "concat" => println!("{}", help::CONCAT_HELP),
                        "copy" => println!("{}", help::COPY_HELP),
                        "document" => println!("{}", help::DOCUMENT_HELP),
                        "merge" => println!("{}", help::MERGE_HELP),
                        "pack" => println!("{}", help::PACK_HELP),
                        "render" => println!("{}", help::RENDER_HELP),
                        "transform" => println!("{}", help::TRANSFORM_HELP),
                        "unpack" => println!("{}", help::UNPACK_HELP),
                        "yaml" => println!("{}", help::YAML_HELP),
                        _ => println!("{}", help::TOPIC_HELP),
                    }
                    std::process::exit(0);
                },
                _ => {},
            }
        },
        _ => {},
    }

    // Transforms the string-based arguments into a unit sequence.
    loop {
        if args.is_empty() {
            break;
        }

        let arg = args.pop_front().unwrap();
        if arg == STDIO_PLACEHOLDER {
            // May be:
            // - stdin before the first command, or
            // - stdout after the last command.
            units.push_back(Unit::File(UnitFile::for_path(STDIO_PLACEHOLDER)));
        } else if arg.starts_with(LONG_OPTION_PREFIX) {
            // Long option.

            if arg.len() <= LONG_OPTION_PREFIX_LEN {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }

            if let Some(option) = arg.get(LONG_OPTION_PREFIX_LEN..) {
                if option == "document" {
                    let hint = args.pop_front().unwrap();
                    let spec = args.pop_front().unwrap();
                    let doc =
                            match UnitDocument::for_hint(&hint, &spec) {
                                Ok(d) => d,
                                Err(_) => {
                                    eprintln!("wrong parameter");
                                    std::process::exit(10);
                                },
                            };
                    units.push_back(Unit::Document(doc));
                } else if option == "check" {
                    units.push_back(Unit::Check);
                } else if option == "copy" {
                    units.push_back(Unit::Copy);
                } else if option == "concat" {
                    units.push_back(Unit::Concat);
                } else if option == "merge" {
                    // With optional depth.
                    let mut ucmd = UnitCommand::default();
                    loop {
                        let next_opt =
                                match args.front() {
                                    Some(o) => o,
                                    None => break,
                                };
                        if next_opt.starts_with("--depth") || next_opt.starts_with("-d") {
                            let opt =
                                    match FlaggedOption::from_str(&args.pop_front().unwrap()) {
                                        Ok(o) => o,
                                        Err(_) => {
                                            eprintln!("wrong parameter");
                                            std::process::exit(10);
                                        },
                                    };
                            match opt.value {
                                Some(v) => {
                                    let depth = v.parse::<isize>().unwrap();
                                    #[cfg(feature = "debug")]
                                    eprintln!("option: depth {}", depth);
                                    ucmd.depth = Some(depth);
                                },
                                None => {
                                    let depth = args.pop_front().unwrap();
                                    let depth = depth.parse::<isize>().unwrap();
                                    #[cfg(feature = "debug")]
                                    eprintln!("option: depth {}", depth);
                                    ucmd.depth = Some(depth);
                                },
                            }
                        } else {
                            break;
                        }
                    } // loop
                    units.push_back(Unit::Merge(ucmd));
                } else if option == "pack" {
                    units.push_back(Unit::Pack);
                } else if option == "unpack" {
                    units.push_back(Unit::Unpack);
                } else if option == "render" {
                    // With mandatory path.
                    let path =
                            match args.pop_front() {
                                Some(p) => p,
                                None => {
                                    eprintln!("missing template path");
                                    std::process::exit(10);
                                },
                            };
                    if path != STDIO_PLACEHOLDER && path.starts_with(STDIO_PLACEHOLDER) {
                        eprintln!("wrong template path");
                        std::process::exit(10);
                    }

                    let ucmd = UnitCommand::for_path(&path);
                    units.push_back(Unit::Render(ucmd));
                } else if option == "transform" {
                    // With mandatory path.
                    let path =
                            match args.pop_front() {
                                Some(p) => p,
                                None => {
                                    eprintln!("missing lua path");
                                    std::process::exit(10);
                                },
                            };
                    if path != STDIO_PLACEHOLDER && path.starts_with(STDIO_PLACEHOLDER) {
                        eprintln!("wrong lua path");
                        std::process::exit(10);
                    }

                    let ucmd = UnitCommand::for_path(&path);
                    units.push_back(Unit::Transform(ucmd));
                } else {
                    // A file format -> input or output.
                    let format =
                            match FileFormat::for_str(option) {
                                Ok(f) => f,
                                Err(_) => {
                                    eprintln!("wrong parameter");
                                    std::process::exit(10);
                                },
                            };

                    let mut ufile = UnitFile::for_format(format);
                    loop {
                        let next_opt =
                                match args.front() {
                                    Some(o) => o,
                                    None => {
                                        eprintln!("missing path");
                                        std::process::exit(10);
                                    },
                                };
                        if next_opt == "--dots" {
                            args.pop_front();
                            ufile.dots = Some(true);
                        } else if next_opt == "--eol" {
                            args.pop_front();
                            ufile.eol = Some(true);
                        } else if next_opt == "--fix" {
                            args.pop_front();
                            ufile.fix = Some(true);
                        } else if next_opt == "--pretty" {
                            args.pop_front();
                            ufile.pretty = Some(true);
                        } else if next_opt.starts_with("--stream") || next_opt.starts_with("-s") {
                            let opt =
                                    match FlaggedOption::from_str(&args.pop_front().unwrap()) {
                                        Ok(o) => o,
                                        Err(_) => {
                                            eprintln!("wrong parameter");
                                            std::process::exit(10);
                                        },
                                    };
                            match opt.value {
                                Some(v) => {
                                    let limit = v.parse::<isize>().unwrap();
                                    #[cfg(feature = "debug")]
                                    eprintln!("option: stream {}", limit);
                                    ufile.stream = Some(limit);
                                },
                                None => {
                                    #[cfg(feature = "debug")]
                                    eprintln!("option: stream");
                                    ufile.stream = Some(-1);
                                },
                            }
                        } else {
                            let path = args.pop_front().unwrap();
                            if path != STDIO_PLACEHOLDER && path.starts_with(STDIO_PLACEHOLDER) {
                                eprintln!("wrong path");
                                std::process::exit(10);
                            }

                            ufile.path = path;
                            break;
                        }
                    } // loop

                    units.push_back(Unit::File(ufile));
                }
            } else {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }
        } else if arg.starts_with(SHORT_OPTION_PREFIX) {
            // Short option.

            let is_long = arg.len() > (SHORT_OPTION_PREFIX_LEN + 1);
            let option = arg.get(SHORT_OPTION_PREFIX_LEN..SHORT_OPTION_PREFIX_LEN + 1).unwrap();

            if option == "C" {
                units.push_back(Unit::Copy);
            } else if option == "D" {
                // A Document.
                if arg.len() > (SHORT_OPTION_PREFIX_LEN + 2) {
                    // Very long. Everything is concatenated.
                    let hint = arg.get(SHORT_OPTION_PREFIX_LEN + 1..SHORT_OPTION_PREFIX_LEN + 2).unwrap();
                    let spec = arg.get(SHORT_OPTION_PREFIX_LEN + 2..).unwrap();
                    let doc =
                            match UnitDocument::for_hint(hint, spec) {
                                Ok(d) => d,
                                Err(_) => {
                                    eprintln!("wrong parameter");
                                    std::process::exit(10);
                                },
                            };
                    units.push_back(Unit::Document(doc));
                } else if is_long {
                    // Only the hint is concatenated.
                    let hint = arg.get(SHORT_OPTION_PREFIX_LEN + 1..).unwrap();
                    let spec = args.pop_front().unwrap();
                    let doc =
                            match UnitDocument::for_hint(hint, &spec) {
                                Ok(d) => d,
                                Err(_) => {
                                    eprintln!("wrong parameter");
                                    std::process::exit(10);
                                },
                            };
                    units.push_back(Unit::Document(doc));
                } else {
                    let hint = args.pop_front().unwrap();
                    let spec = args.pop_front().unwrap();
                    let doc =
                            match UnitDocument::for_hint(&hint, &spec) {
                                Ok(d) => d,
                                Err(_) => {
                                    eprintln!("wrong parameter");
                                    std::process::exit(10);
                                },
                            };
                    units.push_back(Unit::Document(doc));
                }
            } else if option == "R" {
                // With mandatory path.
                let path =
                        match args.pop_front() {
                            Some(p) => p,
                            None => {
                                eprintln!("missing template path");
                                std::process::exit(10);
                            },
                        };
                if path != STDIO_PLACEHOLDER && path.starts_with(STDIO_PLACEHOLDER) {
                    eprintln!("wrong template path");
                    std::process::exit(10);
                }

                let ucmd = UnitCommand::for_path(&path);
                units.push_back(Unit::Render(ucmd));
            } else if option == "T" {
                // With mandatory path.
                let path =
                        match args.pop_front() {
                            Some(p) => p,
                            None => {
                                eprintln!("missing lua path");
                                std::process::exit(10);
                            },
                        };
                if path != STDIO_PLACEHOLDER && path.starts_with(STDIO_PLACEHOLDER) {
                    eprintln!("wrong lua path");
                    std::process::exit(10);
                }

                let ucmd = UnitCommand::for_path(&path);
                units.push_back(Unit::Transform(ucmd));
            } else {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }
        } else {
            // The argment is a path.
            // Input or output.
            units.push_back(Unit::File(UnitFile::for_path(&arg)));
        }
    } // loop

    // Units.
    #[cfg(feature = "debug")]
    for (unit_idx, unit) in units.iter_mut().enumerate() {
        eprintln!("[{}] {:?}", unit_idx, unit);
    } // for

    // (2ofx) Read input documents.
    let mut values = VecDeque::new();
    loop {
        let unit =
                match units.pop_front() {
                    Some(u) => u,
                    None => break,
                };
        match unit {
            Unit::Document(d) => {
                let value =
                        match create_document(d.hint, &d.content) {
                            Ok(v) => v,
                            Err(_) => {
                                eprintln!("wrong input");
                                std::process::exit(21);
                            },
                        };

                values.push_back(value);
            },
            Unit::File(f) => match f.format {
                FileFormat::Unknown => {
                    eprintln!("wrong input");
                    std::process::exit(21);
                },
                FileFormat::Json => {
                    let content =
                            match read_content(&f.path) {
                                Ok(c) => c,
                                Err(e) => panic!("{}", e),
                            };
                    values.push_back(value::from_json_str(&content).unwrap());
                },
                FileFormat::Lua => {
                    let content =
                            match read_content(&f.path) {
                                Ok(c) => c,
                                Err(e) => panic!("{}", e),
                            };
                    values.push_back(create_lua_value(&content).unwrap());
                },
                FileFormat::Toml => {
                    let content =
                            match read_content(&f.path) {
                                Ok(c) => c,
                                Err(e) => panic!("{}", e),
                            };
                    values.push_back(value::from_toml_str(&content).unwrap());
                },
                FileFormat::Yaml => {
                    if f.has_stream() {
                        let content =
                                match read_content(&f.path) {
                                    Ok(c) => c,
                                    Err(e) => panic!("{}", e),
                                };
                        let docs =
                                match yaml::read_stream(&content) {
                                    Ok(c) => c,
                                    Err(_) => {
                                        eprintln!("wrong input");
                                        std::process::exit(21);
                                    },
                                };
                        for doc in docs.iter() {
                            values.push_back(value::from_yaml_str(&doc).unwrap());
                        } // for
                    } else {
                        let content =
                                match read_content(&f.path) {
                                    Ok(c) => c,
                                    Err(e) => panic!("{}", e),
                                };
                        values.push_back(value::from_yaml_str(&content).unwrap());
                    }
                },
            },
            _ => {
                units.push_front(unit);
                break;
            },
        }
    } // loop

    // (3ofx) Process commands.
    loop {
        let unit =
                match units.pop_front() {
                    Some(u) => u,
                    None => break,
                };
        match unit {
            Unit::Copy => {
                // No treatment necessary since every input will be written afterwards.
                break;
            },
            Unit::Concat => {
                let mut res = Vec::new();
                loop {
                    if values.is_empty() {
                        break;
                    }

                    let val = values.pop_front().unwrap();
                    match val {
                        Value::Array(l) => {
                            for e in l.iter() {
                                res.push(e.clone());
                            } // for
                        },
                        _ => {
                            eprintln!("wrong parameter");
                            std::process::exit(21);
                        },
                    }
                } // loop

                values.push_back(Value::Array(res));
            },
            Unit::Check => {
                // Nothing else to do since every input has been read and checked already.
                std::process::exit(0);
            },
            Unit::Merge(c) => {
                let depth =
                        match c.depth {
                            Some(d) => d,
                            None => -1,
                        };
                loop {
                    match values.len() {
                        0 => panic!("cannot merge without any input"),
                        1 => break,
                        _ => {},
                    }

                    let left = values.pop_front().unwrap();
                    let right = values.pop_front().unwrap();
                    let res = value::merge_values(&left, &right, depth);
                    values.push_front(res);
                } // loop
            },
            Unit::Pack => {
                let mut res = Vec::new();
                loop {
                    if values.is_empty() {
                        break;
                    }

                    let val = values.pop_front().unwrap();
                    res.push(val);
                } // loop

                values.push_back(Value::Array(res));
            },
            Unit::Unpack => {
                let mut len = values.len();
                loop {
                    if len == 0 {
                        break;
                    }

                    len -= 1;
                    let val = values.pop_front().unwrap();
                    match val {
                        Value::Array(l) => {
                            for e in l.iter() {
                                values.push_back(e.clone());
                            } // for
                        },
                        _ => {
                            eprintln!("wrong parameter");
                            std::process::exit(21);
                        },
                    }
                } // loop
            },
            Unit::Render(c) => {
                let template = template::Template::for_path(c.path.as_ref().unwrap());
                let lua = rlua::Lua::new();
                let input_values = values.clone();
                values.clear();
                let output_value =
                        lua.context(|lua_ctx| {
                            match lua_ctx.load(command::LUA_PRELUDE).exec() {
                                Ok(_) => {},
                                Err(e) => panic!("{}", e),
                            }

                            let globals = lua_ctx.globals();

                            let ctx: rlua::Table =
                                    match globals.get("ctx") {
                                        Ok(v) => v,
                                        Err(e) => panic!("{}", e),
                                    };

                            for (_, value) in input_values.iter().enumerate() {
                                let mut sb = String::new();
                                sb.push_str("table.insert(ctx.inputs,");
                                sb.push_str(&value::to_lua_string(&value));
                                sb.push_str(")");
                                lua_ctx.load(&sb).exec().unwrap();
                            } // for

                            match lua_ctx.load(&template.content).exec() {
                                Ok(_) => {},
                                Err(e) => panic!("{}", e),
                            }

                            let outputs: rlua::Table =
                                    match ctx.get("outputs") {
                                        Ok(v) => v,
                                        Err(e) => panic!("{}", e),
                                    };

                            match outputs.get(1).unwrap() {
                                rlua::Value::Table(t) => Some(value::from_processed_template(t.clone())),
                                _ => None,
                            }
                        });
                if let Some(v) = output_value {
                    values.push_back(Value::String(v));
                }
            },
            Unit::Transform(c) => {
                let lua_content =
                        match std::fs::read_to_string(c.path.as_ref().unwrap()) {
                            Ok(c) => c,
                            Err(e) => panic!("{}", e),
                        };
                let lua = rlua::Lua::new();
                let input_values = values.clone();
                values.clear();
                let output_value =
                        lua.context(|lua_ctx| {
                            match lua_ctx.load(command::LUA_PRELUDE).exec() {
                                Ok(_) => {},
                                Err(e) => panic!("{}", e),
                            }

                            let globals = lua_ctx.globals();

                            let ctx: rlua::Table =
                                    match globals.get("ctx") {
                                        Ok(v) => v,
                                        Err(e) => panic!("{}", e),
                                    };

                            for (_, value) in input_values.iter().enumerate() {
                                let mut sb = String::new();
                                sb.push_str("table.insert(ctx.inputs,");
                                sb.push_str(&value::to_lua_string(&value));
                                sb.push_str(")");
                                lua_ctx.load(&sb).exec().unwrap();
                            } // for

                            match lua_ctx.load(&lua_content).exec() {
                                Ok(_) => {},
                                Err(e) => panic!("{}", e),
                            }

                            let outputs: rlua::Table =
                                    match ctx.get("outputs") {
                                        Ok(v) => v,
                                        Err(e) => panic!("{}", e),
                                    };

                            value::from_lua_table(outputs)
                        });
                if let Value::Array(vals) = output_value {
                    values.extend(vals);
                }
            },
            _ => {
                units.push_front(unit);
                break;
            },
        }
    } // loop

    // (4ofx) Write output documents.
    loop {
        let unit =
                match units.pop_front() {
                    Some(u) => u,
                    None => break,
                };
        match unit {
            Unit::File(f) => {
                // The number of values to process.
                let mut val_cnt =
                        match f.stream {
                            Some(s) => s,
                            None => 1,
                        };
                let mut output_content = String::new();
                while val_cnt != 0 {
                    let val =
                            match values.pop_front() {
                                Some(v) => v,
                                None => {
                                    if val_cnt < 0 {
                                        // Infinite stream: stop because there is no more value to process.
                                        break;
                                    } else {
                                        // Finite stream: error.
                                        eprintln!("wrong output");
                                        std::process::exit(31);
                                    }
                                }
                            };
                    if val_cnt > 0 {
                        val_cnt -= 1;
                    }
                    match f.format {
                        FileFormat::Unknown => {
                            let buf =
                                    match val {
                                        Value::Nil => "~".to_owned(),
                                        Value::Boolean(v) => format!("{}", v),
                                        Value::Integer(v) => format!("{}", v),
                                        Value::Float(v) => format!("{}", v),
                                        Value::String(v) => v.clone(),
                                        _ => panic!("wtf"),
                                    };
                            output_content.push_str(&buf);
                        },
                        FileFormat::Json => {
                            let buf =
                                    if f.has_pretty() {
                                        match serde_json::to_string_pretty(&val) {
                                            Ok(c) => c,
                                            Err(e) => panic!("{}", e),
                                        }
                                    } else {
                                        match serde_json::to_string(&val) {
                                            Ok(c) => c,
                                            Err(e) => panic!("{}", e),
                                        }
                                    };
                            output_content.push_str(&buf);
                        },
                        FileFormat::Lua => {
                            let buf = value::to_lua_string(&val);
                            output_content.push_str(&buf);
                        },
                        FileFormat::Toml => {
                            let fixed_val =
                                    if f.has_fix() {
                                        value::fix_toml(&val)
                                    } else {
                                        val
                                    };
                            let buf =
                                    match toml::to_string(&fixed_val) {
                                        Ok(c) => c,
                                        Err(e) => panic!("{}", e),
                                    };
                            output_content.push_str(&buf);
                        },
                        FileFormat::Yaml => {
                            let buf =
                                    match serde_yaml::to_string(&val) {
                                        Ok(c) => c,
                                        Err(e) => panic!("{}", e),
                                    };
                            output_content.push_str(&buf);
                            if f.has_dots() {
                                output_content.push_str("...\n");
                            }
                        },
                    } // match f.format
                    if f.has_eol() && !output_content.ends_with("\n") {
                        output_content.push('\n');
                    }
                } // while
                if f.path == STDIO_PLACEHOLDER {
                    print!("{}", output_content);
                } else {
                    match std::fs::write(f.path, output_content) {
                        Ok(_) => {},
                        Err(e) => panic!("{}", e),
                    }
                }
            },
            _ => {
                units.push_front(unit);
                break;
            },
        }
    } // loop

    if !values.is_empty() || !units.is_empty() {
        eprintln!("no output");
        std::process::exit(30);
    }
}
