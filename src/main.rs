extern crate indexmap;
extern crate rlua;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

mod unit;
mod value;
mod command;

use crate::unit::{
    FileFormat,
    UnitFile,
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
    let args: Vec<String> = std::env::args().skip(1).collect();
    // let mut input_format: Option<Format> = None;
    // let input_path = args[1].to_owned();
    let mut units: Vec<Unit> = Vec::new();

    for arg in args {
        if arg == STDIO_PLACEHOLDER {
            if let Some(u) = units.last_mut() {
                if let Unit::File(f) = u {
                    if f.path.is_empty() {
                        f.path = STDIO_PLACEHOLDER.to_owned();
                    } else {
                        units.push(Unit::File(UnitFile::for_path(STDIO_PLACEHOLDER)));
                    }
                } else {
                    units.push(Unit::File(UnitFile::for_path(STDIO_PLACEHOLDER)));
                }
            } else {
                units.push(Unit::File(UnitFile::for_path(STDIO_PLACEHOLDER)));
            }
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
                } else if option == "pretty" {
                    if let Some(u) = units.last_mut() {
                        if let Unit::File(f) = u {
                            // The last unit is a file.
                            if !f.path.is_empty() {
                                // The file is complete.
                                let mut file = UnitFile::default();
                                file.pretty = Some(true);
                                units.push(Unit::File(file));
                            } else {
                                f.pretty = Some(true);
                            }
                        } else {
                            // The last unit is not a file.
                            let mut file = UnitFile::default();
                            file.pretty = Some(true);
                            units.push(Unit::File(file));
                        }
                    } else {
                        // First unit.
                        let mut file = UnitFile::default();
                        file.pretty = Some(true);
                        units.push(Unit::File(file));
                    }
                } else {
                    // Try a file format.
                    let format = FileFormat::for_str(option);
                    if format.is_known() {
                        if let Some(u) = units.last_mut() {
                            if let Unit::File(f) = u {
                                // The last unit is a file.
                                if !f.path.is_empty() {
                                    // The file is complete.
                                    units.push(Unit::File(UnitFile::for_format(format)));
                                } else if f.format != FileFormat::Unknown {
                                    eprintln!("wrong parameter");
                                    std::process::exit(10);
                                } else {
                                    f.format = format;
                                }
                            } else {
                                // The last unit is not a file.
                                units.push(Unit::File(UnitFile::for_format(format)));
                            }
                        } else {
                            // First unit.
                            units.push(Unit::File(UnitFile::for_format(format)));
                        }
                    } else {
                        eprintln!("wrong parameter");
                        std::process::exit(10);
                    }
                }
            } else {
                panic!("internal error");
            }
        } else if arg.starts_with(SHORT_OPTION_PREFIX) {
            if arg.len() != (SHORT_OPTION_PREFIX_LEN + 1) {
                eprintln!("wrong parameter");
                std::process::exit(10);
            }

            // if let Some(option) = arg.get(SHORT_OPTION_PREFIX_LEN..) {

            // }
            eprintln!("not implemented yet");
        } else {
            // The argment is a path.
            if let Some(u) = units.last_mut() {
                if let Unit::File(f) = u {
                    // The last unit is a file.
                    if !f.path.is_empty() {
                        // The file is complete.
                        units.push(Unit::File(UnitFile::for_path(&arg)));
                    } else {
                        f.path = arg;
                    }
                } else {
                    units.push(Unit::File(UnitFile::for_path(&arg)));
                }
            } else {
                units.push(Unit::File(UnitFile::for_path(&arg)));
            }
        }
    } // for
    if let Some(u) = units.last_mut() {
        if let Unit::File(f) = u {
            if f.path.is_empty() {
                f.path = STDIO_PLACEHOLDER.to_owned();
            }
        }
    }

    let mut any_command = false;
    let mut any_lua = false;
    for (unit_idx, unit) in units.iter().enumerate() {

        eprintln!("[{}] {:?}", unit_idx, unit); // FIXME

        if let Unit::File(f) = unit {
            if f.format == FileFormat::Lua {
                any_command = true;
                any_lua = true;
            }
        }
    }

    if !any_command {
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

    if any_lua {
        // Lua processing.
        let mut lua_processed = false;
        let mut values = Vec::new();
        for (_unit_idx, unit) in units.iter().enumerate() {
            if let Unit::File(f) = unit {
                match f.format {
                    FileFormat::Json => {
                        if lua_processed {
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
                        if lua_processed {
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

                            // values.push(value::from_yaml_str(&content).unwrap());
                        }
                    },
                    FileFormat::Lua => {
                        let lua_content =
                                match std::fs::read_to_string(&f.path) {
                                    Ok(c) => c,
                                    Err(e) => panic!("{}", e),
                                };
                        let lua = rlua::Lua::new();
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

                                    match lua_ctx.load(&lua_content).exec() {
                                        Ok(_) => {},
                                        Err(e) => panic!("{}", e),
                                    }

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
                        lua_processed = true;
                        if let Some(v) = output_value {
                            values.push(v);
                        }
                    },
                    _ => {},
                };
            }
        } // for
        std::process::exit(0);
    }
}
