extern crate indexmap;
extern crate rlua;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

use std::collections::VecDeque;

mod unit;
mod value;
mod command;
mod template;

use crate::unit::{
    LUA_PATH_SUFFIX,
    FileFormat,
    UnitFile,
    UnitCommand,
    Unit,
};

const STDIO_PLACEHOLDER: &str = "-";

const LONG_OPTION_PREFIX: &str = "--";
const LONG_OPTION_PREFIX_LEN: usize = LONG_OPTION_PREFIX.len();

const SHORT_OPTION_PREFIX: &str = "-";
const SHORT_OPTION_PREFIX_LEN: usize = SHORT_OPTION_PREFIX.len();

const HELP_CMD: &str = "--help";
const VERSION_CMD: &str = "--version";

const PROGRAM: &str = "rmarshal";
const VERSION: &str = "0.1.0";

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

/**
 * Exit codes:
 * - INTERNAL_ERROR(1)
 * - WRONG_PARAMETER(10)
 * - UNKNOWN_FILE_FORMAT(11)
 * - NO_INPUT(20)
 * - WRONG_INPUT(21)
 */
fn main() {
    // (1ofx) Parse arguments.
    let mut units: VecDeque<Unit> = VecDeque::new();
    let mut args: VecDeque<String> = std::env::args().skip(1).collect();
    match args.len() {
        0 => {
            eprintln!("wrong parameter");
            std::process::exit(10);
        },
        1 => {
            match args.front().unwrap().as_str() {
                HELP_CMD => {
                    println!("Usage: {} [INPUT...] COMMAND [OUTPUT...]", PROGRAM);
                    std::process::exit(0);
                },
                VERSION_CMD => {
                    println!("{} {}", PROGRAM, VERSION);
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
            // For Lua and Template.
            units.push_back(Unit::File(UnitFile::for_path(STDIO_PLACEHOLDER)));
        } else if arg.starts_with(LONG_OPTION_PREFIX) {
            // Long option.

            if arg.len() <= LONG_OPTION_PREFIX_LEN {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }

            if let Some(option) = arg.get(LONG_OPTION_PREFIX_LEN..) {
                if option == "check" {
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
                        if next_opt == "--depth" || next_opt == "-d" {
                            args.pop_front();
                            let depth = args.pop_front().unwrap();
                            let depth = depth.parse::<isize>().unwrap();
                            ucmd.depth = Some(depth);
                        } else {
                            break;
                        }
                    } // loop
                    units.push_back(Unit::Merge(ucmd));
                } else if option == "pack" {
                    units.push_back(Unit::Pack);
                } else if option == "unpack" {
                    units.push_back(Unit::Unpack);
                } else if option == "lua" {
                    // With mandatory path.
                    let path =
                            match args.pop_front() {
                                Some(p) => p,
                                None => {
                                    eprintln!("missing lua path");
                                    std::process::exit(10);
                                },
                            };
                    let ucmd = UnitCommand::for_path(&path);
                    units.push_back(Unit::Lua(ucmd));
                } else if option == "template" {
                    // With mandatory path.
                    let path =
                            match args.pop_front() {
                                Some(p) => p,
                                None => {
                                    eprintln!("missing template path");
                                    std::process::exit(10);
                                },
                            };
                    let ucmd = UnitCommand::for_path(&path);
                    units.push_back(Unit::Template(ucmd));
                } else {
                    // A file format -> input or output.
                    let format = FileFormat::for_str(option);
                    if !format.is_known() {
                        eprintln!("wrong parameter");
                        std::process::exit(10);
                    }

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
                        if next_opt == "--end" {
                            args.pop_front();
                            ufile.end = Some(true);
                        } else if next_opt == "--fix" {
                            args.pop_front();
                            ufile.fix = Some(true);
                        } else if next_opt == "--pretty" {
                            args.pop_front();
                            ufile.pretty = Some(true);
                        } else {
                            ufile.path = args.pop_front().unwrap();
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

            if arg.len() != (SHORT_OPTION_PREFIX_LEN + 1) {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }

            if let Some(_option) = arg.get(SHORT_OPTION_PREFIX_LEN..) {
                eprintln!("not implemented yet");
                std::process::exit(10);
            } else {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }
        } else {
            // The argment is a path.
            let lc_path = arg.to_lowercase();
            if lc_path.ends_with(LUA_PATH_SUFFIX) {
                units.push_back(Unit::Lua(UnitCommand::for_path(&arg)));
            } else {
                // Input or output.
                units.push_back(Unit::File(UnitFile::for_path(&arg)));
            }
        }
    } // loop

    // Debug.
    for (unit_idx, unit) in units.iter_mut().enumerate() {
        eprintln!("[{}] {:?}", unit_idx, unit); // FIXME
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
            Unit::File(f) => match f.format {
                FileFormat::Unknown => panic!("wtf"),
                FileFormat::Json => {
                    let content =
                            match read_content(&f.path) {
                                Ok(c) => c,
                                Err(e) => panic!("{}", e),
                            };

                    values.push_back(value::from_json_str(&content).unwrap());
                },
                FileFormat::Yaml => {
                    let content =
                            match read_content(&f.path) {
                                Ok(c) => c,
                                Err(e) => panic!("{}", e),
                            };

                    values.push_back(value::from_yaml_str(&content).unwrap());
                },
                FileFormat::Toml => {
                    let content =
                            match read_content(&f.path) {
                                Ok(c) => c,
                                Err(e) => panic!("{}", e),
                            };

                    values.push_back(value::from_toml_str(&content).unwrap());
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
                        value::Value::Array(l) => {
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

                values.push_back(value::Value::Array(res));
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

                values.push_back(value::Value::Array(res));
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
                        value::Value::Array(l) => {
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
            Unit::Lua(c) => {
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
                                // println!("{}", sb);
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
                if let value::Value::Array(vals) = output_value {
                    values.extend(vals);
                }
            },
            Unit::Template(c) => {
                let template = template::Template::for_path(c.path.as_ref().unwrap());
                // println!("{}", template.content);
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
                                // println!("{}", sb);
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
                    values.push_back(value::Value::String(v));
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
            Unit::File(f) => match f.format {
                FileFormat::Unknown => {
                    let v = values.pop_front().unwrap();
                    if let value::Value::String(s) = v {
                        if f.path == STDIO_PLACEHOLDER {
                            print!("{}", s);
                        } else {
                            match std::fs::write(f.path, s) {
                                Ok(_) => {},
                                Err(e) => panic!("{}", e),
                            }
                        }
                    } else {
                        panic!("wtf");
                    }
                },
                FileFormat::Json => {
                    let v = values.pop_front().unwrap();
                    let mut output_content =
                            match f.pretty {
                                Some(true) => match serde_json::to_string_pretty(&v) {
                                    Ok(c) => c,
                                    Err(e) => panic!("{}", e),
                                },
                                _ => match serde_json::to_string(&v) {
                                    Ok(c) => c,
                                    Err(e) => panic!("{}", e),
                                },
                            };
                    output_content.push('\n');
                    if f.path == STDIO_PLACEHOLDER {
                        print!("{}", output_content);
                    } else {
                        match std::fs::write(f.path, output_content) {
                            Ok(_) => {},
                            Err(e) => panic!("{}", e),
                        }
                    }
                },
                FileFormat::Yaml => {
                    let v = values.pop_front().unwrap();
                    let mut output_content =
                            match serde_yaml::to_string(&v) {
                                Ok(c) => c,
                                Err(e) => panic!("{}", e),
                            };
                    if let Some(true) = f.end {
                        output_content.push_str("...\n");
                    }
                    if f.path == STDIO_PLACEHOLDER {
                        print!("{}", output_content);
                    } else {
                        match std::fs::write(f.path, output_content) {
                            Ok(_) => {},
                            Err(e) => panic!("{}", e),
                        }
                    }
                },
                FileFormat::Toml => {
                    let v = values.pop_front().unwrap();
                    let v =
                            if let Some(true) = f.fix {
                                value::fix_toml(&v)
                            } else {
                                v
                            };
                    let output_content =
                            match toml::to_string(&v) {
                                Ok(c) => c,
                                Err(e) => panic!("{}", e),
                            };
                    if f.path == STDIO_PLACEHOLDER {
                        print!("{}", output_content);
                    } else {
                        match std::fs::write(f.path, output_content) {
                            Ok(_) => {},
                            Err(e) => panic!("{}", e),
                        }
                    }
                },
            },
            _ => {
                units.push_front(unit);
                break;
            },
        }
    } // loop
}
