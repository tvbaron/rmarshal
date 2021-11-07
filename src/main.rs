extern crate indexmap;
extern crate rlua;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

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
    let mut units: Vec<Unit> = Vec::new();
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        if arg == STDIO_PLACEHOLDER {
            if let Some(last_unit) = units.last_mut() {
                if let Unit::File(f) = last_unit {
                    if f.path.is_empty() {
                        f.path = STDIO_PLACEHOLDER.to_owned();
                        continue;
                    }
                }
            }

            units.push(Unit::File(UnitFile::for_path(STDIO_PLACEHOLDER)));
        } else if arg.starts_with(LONG_OPTION_PREFIX) {
            if arg.len() <= LONG_OPTION_PREFIX_LEN {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }

            if let Some(option) = arg.get(LONG_OPTION_PREFIX_LEN..) {
                if option == "check" {
                    units.push(Unit::Check);
                } else if option == "merge" {
                    units.push(Unit::Merge);
                } else if option == "lua" {
                    units.push(Unit::Lua(UnitCommand::default()));
                } else if option == "template" {
                    units.push(Unit::Template(UnitCommand::default()));
                } else if option == "pretty" {
                    if let Some(last_unit) = units.last_mut() {
                        if let Unit::File(f) = last_unit {
                            // The last unit is a file.
                            if f.path.is_empty() {
                                f.pretty = Some(true);
                                continue;
                            }
                        }
                    }

                    let mut file = UnitFile::default();
                    file.pretty = Some(true);
                    units.push(Unit::File(file));
                } else {
                    // Try a file format.
                    let format = FileFormat::for_str(option);
                    if format.is_known() {
                        if let Some(last_unit) = units.last_mut() {
                            if let Unit::File(f) = last_unit {
                                // The last unit is a file.
                                if f.path.is_empty() {
                                    f.format = format;
                                    continue;
                                }
                            }
                        }

                        units.push(Unit::File(UnitFile::for_format(format)));
                    } else {
                        eprintln!("wrong parameter");
                        std::process::exit(10);
                    }
                }
            } else {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }
        } else if arg.starts_with(SHORT_OPTION_PREFIX) {
            if arg.len() != (SHORT_OPTION_PREFIX_LEN + 1) {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }

            if let Some(_option) = arg.get(SHORT_OPTION_PREFIX_LEN..) {
                eprintln!("not implemented yet");
            } else {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }
        } else {
            // The argment is a path.
            if let Some(last_unit) = units.last_mut() {
                match last_unit {
                    Unit::File(f) => {
                        // The last unit is a file.
                        if f.path.is_empty() {
                            // The file is not complete.
                            f.path = arg;
                            continue;
                        }
                    },
                    Unit::Lua(c) | Unit::Template(c) => {
                        // The last unit is a lua/template command.
                        if let None = c.path {
                            c.path = Some(arg);
                            continue;
                        }
                    },
                    _ => {},
                } // match
            }

            let lc_path = arg.to_lowercase();
            if lc_path.ends_with(LUA_PATH_SUFFIX) {
                units.push(Unit::Lua(UnitCommand::for_path(&arg)));
            } else {
                units.push(Unit::File(UnitFile::for_path(&arg)));
            }
        }
    } // for
    // if let Some(u) = units.last_mut() {
    //     if let Unit::File(f) = u {
    //         if f.path.is_empty() {
    //             f.path = STDIO_PLACEHOLDER.to_owned();
    //         }
    //     }
    // }

    // (2ofx) Check and sanitize units.
    let mut output_cnt = 0;
    let mut any_command = false;
    for (unit_idx, unit) in units.iter_mut().enumerate() {

        eprintln!("[{}] {:?}", unit_idx, unit); // FIXME

        match unit {
            Unit::File(f) => {
                if any_command {
                    if output_cnt > 0 {
                        eprintln!("wrong parameter");
                        std::process::exit(10);
                    }

                    output_cnt += 1;
                    if f.path.is_empty() {
                        f.path = STDIO_PLACEHOLDER.to_owned();
                    }
                    if f.format == FileFormat::Unknown {

                    }
                }
            },
            Unit::Check | Unit::Merge | Unit::Lua(_) | Unit::Template(_) => {
                any_command = true;
            },
        }
    } // for

    if any_command {
        // Command processing.
        let mut command_processed = false;
        let mut values = Vec::new();
        for (_, unit) in units.iter().enumerate() {
            match unit {
                Unit::File(f) => match f.format {
                    FileFormat::Unknown => {
                        if command_processed {
                            let v = &values[0];
                            match v {
                                value::Value::String(s) => {
                                    print!("{}", s);
                                },
                                _ => panic!("wtf"),
                            }
                        } else {
                            panic!("wtf");
                        }
                    },
                    FileFormat::Json => {
                        if command_processed {
                            let v = &values[0];
                            let output_content =
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
                            println!("{}", output_content);
                        } else {
                            let content =
                                    match std::fs::read_to_string(&f.path) {
                                        Ok(c) => c,
                                        Err(e) => panic!("{}", e),
                                    };

                            values.push(value::from_json_str(&content).unwrap());
                        }
                    },
                    FileFormat::Yaml => {
                        if command_processed {
                            let v = &values[0];
                            let output_content =
                                    match serde_yaml::to_string(&v) {
                                        Ok(c) => c,
                                        Err(e) => panic!("{}", e),
                                    };
                            println!("{}", output_content);
                        } else {
                            let content =
                                    match std::fs::read_to_string(&f.path) {
                                        Ok(c) => c,
                                        Err(e) => panic!("{}", e),
                                    };

                            values.push(value::from_yaml_str(&content).unwrap());
                        }
                    },
                    _ => {
                        panic!("not implemented yet");
                    },
                },
                Unit::Check => {
                    command_processed = true;
                },
                Unit::Merge => {
                    let lua = rlua::Lua::new();
                    let input_values = values.clone();
                    values.clear();
                    let output_value =
                            lua.context(|lua_ctx| {
                                match lua_ctx.load(command::LUA_PRELUDE).exec() {
                                    Ok(_) => {},
                                    Err(e) => panic!("{}", e),
                                } // match

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
                                    println!("{}", sb);
                                    lua_ctx.load(&sb).exec().unwrap();
                                } // for

                                lua_ctx.load("ctx:set_output(ctx:merge_inputs())").exec().unwrap();

                                let output: rlua::Value =
                                        match ctx.get("output") {
                                            Ok(v) => v,
                                            Err(e) => panic!("{}", e),
                                        };

                                match output {
                                    rlua::Value::Table(t) => Some(value::from_lua_table(t.clone())),
                                    _ => None,
                                }
                            });
                    command_processed = true;
                    if let Some(v) = output_value {
                        values.push(v);
                    }
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
                                } // match

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
                                } // match

                                let output: rlua::Value =
                                        match ctx.get("output") {
                                            Ok(v) => v,
                                            Err(e) => panic!("{}", e),
                                        };

                                match output {
                                    rlua::Value::Table(t) => Some(value::from_lua_table(t.clone())),
                                    _ => None,
                                }
                            });
                    command_processed = true;
                    if let Some(v) = output_value {
                        values.push(v);
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
                                } // match

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
                                } // match

                                let output: rlua::Value =
                                        match ctx.get("output") {
                                            Ok(v) => v,
                                            Err(e) => panic!("{}", e),
                                        };

                                match output {
                                    rlua::Value::Table(t) => Some(value::from_processed_template(t.clone())),
                                    _ => None,
                                }
                            });
                    command_processed = true;
                    if let Some(v) = output_value {
                        values.push(value::Value::String(v));
                    }
                },
            } // match
        } // for
        std::process::exit(0);
    }

    // No command: input -> output.
    if units.len() != 2 {
        panic!("wrong paramters");
    }

    let input_unit = &units[0];
    let input_value =
            match input_unit {
                Unit::File(f) => match f.format {
                    FileFormat::Json => {
                        let content =
                                match std::fs::read_to_string(&f.path) {
                                    Ok(c) => c,
                                    Err(e) => panic!("{}", e),
                                };

                        value::from_json_str(&content).unwrap()
                    },
                    FileFormat::Yaml => {
                        let content =
                                match std::fs::read_to_string(&f.path) {
                                    Ok(c) => c,
                                    Err(e) => panic!("{}", e),
                                };

                        value::from_yaml_str(&content).unwrap()
                    },
                    _ => {
                        eprintln!("wrong input");
                        std::process::exit(21);
                    },
                },
                _ => {
                    eprintln!("no input");
                    std::process::exit(20);
                },
            };

    let output_unit = &units[1];
    let output_content =
            match output_unit {
                Unit::File(f) => match f.format {
                    FileFormat::Json => match f.pretty {
                        Some(true) => match serde_json::to_string_pretty(&input_value) {
                            Ok(c) => c,
                            Err(e) => panic!("{}", e),
                        },
                        _ => match serde_json::to_string(&input_value) {
                            Ok(c) => c,
                            Err(e) => panic!("{}", e),
                        },
                    },
                    FileFormat::Toml => match toml::to_string(&input_value) {
                        Ok(c) => c,
                        Err(e) => panic!("{}", e),
                    },
                    FileFormat::Yaml => match serde_yaml::to_string(&input_value) {
                        Ok(c) => c,
                        Err(e) => panic!("{}", e),
                    },
                    _ => panic!("unknown output format"),
                },
                _ => panic!("wrong unit"),
            };

    println!("{}", output_content);

    std::process::exit(0);
}
