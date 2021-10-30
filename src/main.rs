extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use serde::ser::Serialize;
use serde_json::{Value as JsonValue};
use serde_yaml::{Value as YamlValue};

mod unit;

use crate::unit::{
    FileFormat,
    UnitFile,
    Unit,
};

enum Value {
    Json(JsonValue),
    Yaml(YamlValue),
}

const STDIO_PLACEHOLDER: &str = "-";

const LONG_OPTION_PREFIX: &str = "--";
const LONG_OPTION_PREFIX_LEN: usize = LONG_OPTION_PREFIX.len();

const SHORT_OPTION_PREFIX: &str = "-";
const SHORT_OPTION_PREFIX_LEN: usize = SHORT_OPTION_PREFIX.len();

fn load_json(path: &str) -> JsonValue {
    let content =
            match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => panic!("{}", e),
            };

    match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}

fn load_yaml(path: &str) -> YamlValue {
    let content =
            match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => panic!("{}", e),
            };

    match serde_yaml::from_str(&content) {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    }
}

// fn is_last_unit_file(units: &Vec<Unit>, format_known: bool, complete: bool) -> bool {
//     if let Some(u) = units.last() {
//         if let Unit::File { format: f, path: p } = u {
//             return (!format_known || f != &Format::Unknown) && (!complete && !p.is_empty());
//         }
//     }
//     false
// }

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

    // eprintln!("{:?}", units);

    for (unit_idx, unit) in units.iter().enumerate() {
        eprintln!("[{}] {:?}", unit_idx, unit);
    }

    // (2ofx) Process.
    if units.len() == 2 {
        let input_unit = &units[0];
        let input_content =
                match input_unit {
                    Unit::File(f) => match f.format {
                        FileFormat::Json => Value::Json(load_json(&f.path)),
                        FileFormat::Yaml => Value::Yaml(load_yaml(&f.path)),
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
        match input_content {
            Value::Json(v) => {
                let output_content =
                        match output_unit {
                            Unit::File(f) => match f.format {
                                FileFormat::Json | FileFormat::Unknown => match f.pretty {
                                    Some(true) => match serde_json::to_string_pretty(&v) {
                                        Ok(c) => c,
                                        Err(e) => panic!("{}", e),
                                    },
                                    _ => match serde_json::to_string(&v) {
                                        Ok(c) => c,
                                        Err(e) => panic!("{}", e),
                                    },
                                },
                                FileFormat::Yaml => match serde_yaml::to_string(&v) {
                                    Ok(c) => c,
                                    Err(e) => panic!("{}", e),
                                },
                                _ => panic!("json -> ?"),
                            },
                            _ => panic!("json -> ?"),
                        };

                println!("{}", output_content);
            },
            Value::Yaml(v) => {
                let output_content =
                        match output_unit {
                            Unit::File(f) => match f.format {
                                FileFormat::Json => match f.pretty {
                                    Some(true) => match serde_json::to_string_pretty(&v) {
                                        Ok(c) => c,
                                        Err(e) => panic!("{}", e),
                                    },
                                    _ => match serde_json::to_string(&v) {
                                        Ok(c) => c,
                                        Err(e) => panic!("{}", e),
                                    },
                                },
                                FileFormat::Yaml | FileFormat::Unknown => match serde_yaml::to_string(&v) {
                                    Ok(c) => c,
                                    Err(e) => panic!("{}", e),
                                },
                                _ => panic!("json -> ?"),
                            },
                            _ => panic!("json -> ?"),
                        };

                println!("{}", output_content);
            },
            _ => {
                eprintln!("wrong input");
                std::process::exit(21);
            },
        };
    }
}
