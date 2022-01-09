const JSON_PATH_SUFFIX: &str = ".json";
const LUA_PATH_SUFFIX: &str = ".lua";
const TOML_PATH_SUFFIX: &str = ".toml";
const YAML_PATH_SUFFIX: &str = ".yaml";

#[derive(Debug, PartialEq, Eq)]
pub enum FileFormat {
    Plain,
    Json,
    Lua,
    Toml,
    Yaml,
}

impl Default for FileFormat {
    fn default() -> Self {
        FileFormat::Plain
    }
}

impl FileFormat {
    pub fn for_path(path: &str) -> Self {
        let lc_path = path.to_lowercase();
        if lc_path.ends_with(JSON_PATH_SUFFIX) {
            FileFormat::Json
        } else if lc_path.ends_with(LUA_PATH_SUFFIX) {
            FileFormat::Lua
        } else if lc_path.ends_with(TOML_PATH_SUFFIX) {
            FileFormat::Toml
        } else if lc_path.ends_with(YAML_PATH_SUFFIX) {
            FileFormat::Yaml
        } else {
            FileFormat::Plain
        }
    }

    // Returns a FileFormat for a given string representation.
    pub fn for_str(format: &str) -> Result<Self, ()> {
        match format {
            "plain" => Ok(FileFormat::Plain),
            "json" => Ok(FileFormat::Json),
            "lua" => Ok(FileFormat::Lua),
            "toml" => Ok(FileFormat::Toml),
            "yaml" => Ok(FileFormat::Yaml),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum DocumentHint {
    Any,
    Nil,
    Boolean,
    Integer,
    Float,
    String,
    Json,
    Lua,
}

impl DocumentHint {
    // Returns a DocumentHint for a given string representation.
    pub fn for_str(hint: &str) -> Result<Self, ()> {
        let hint =
                if hint.len() > 1 {
                    hint.to_lowercase()
                } else {
                    hint.to_owned()
                };
        match hint.as_str() {
            "_" | "any" => Ok(DocumentHint::Any),
            "N" | "nil" => Ok(DocumentHint::Nil),
            "B" | "boolean" => Ok(DocumentHint::Boolean),
            "I" | "integer" => Ok(DocumentHint::Integer),
            "F" | "float" => Ok(DocumentHint::Float),
            "S" | "string" => Ok(DocumentHint::String),
            "J" | "json" => Ok(DocumentHint::Json),
            "L" | "lua" => Ok(DocumentHint::Lua),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct UnitDocument {
    pub hint: DocumentHint,
    pub content: String,
}

impl UnitDocument {
    pub fn new(hint: DocumentHint, content: &str) -> Self {
        UnitDocument {
            hint,
            content: content.to_owned(),
        }
    }

    pub fn for_hint(hint: &str, content: &str) -> Result<Self, ()> {
        let hint = DocumentHint::for_str(hint)?;

        Ok(UnitDocument::new(hint, content))
    }
}

#[derive(Debug, Default)]
pub struct UnitFile {
    pub path: String,
    pub format: FileFormat,
    // The ending 3 dots of YAML.
    pub dots: Option<bool>,
    // The trailing new line character.
    pub eol: Option<bool>,
    // To reorder object elements of TOML.
    pub fix: Option<bool>,
    // The JSON pretty format.
    pub pretty: Option<bool>,
    // For multiple documents within the same file.
    pub stream: Option<isize>,
}

impl UnitFile {
    pub fn for_path(path: &str) -> Self {
        UnitFile {
            path: path.to_owned(),
            format: FileFormat::for_path(path),
            dots: None,
            eol: None,
            fix: None,
            pretty: None,
            stream: None,
        }
    }

    // pub fn for_path_format(path: &str, format: FileFormat) -> Self {
    //     UnitFile {
    //         path: path.to_owned(),
    //         format,
    //         pretty: None,
    //     }
    // }

    pub fn for_format(format: FileFormat) -> Self {
        UnitFile {
            path: String::new(),
            format,
            dots: None,
            eol: None,
            fix: None,
            pretty: None,
            stream: None,
        }
    }

    pub fn has_dots(&self) -> bool {
        if let Some(true) = self.dots {
            true
        } else {
            false
        }
    }

    pub fn has_eol(&self) -> bool {
        if let Some(true) = self.eol {
            true
        } else {
            false
        }
    }

    pub fn has_fix(&self) -> bool {
        if let Some(true) = self.fix {
            true
        } else {
            false
        }
    }

    pub fn has_pretty(&self) -> bool {
        if let Some(true) = self.pretty {
            true
        } else {
            false
        }
    }

    pub fn has_stream(&self) -> bool {
        if let Some(_) = self.stream {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct UnitCommand {
    // For Lua and Template commands.
    pub path: Option<String>,
    // For merge command.
    pub depth: Option<isize>,
}

impl UnitCommand {
    pub fn for_path(path: &str) -> Self {
        UnitCommand {
            path: Some(path.to_owned()),
            depth: None,
        }
    }

    // pub fn for_depth(depth: isize) -> Self {
    //     UnitCommand {
    //         path: None,
    //         depth: Some(depth),
    //     }
    // }
}

// Parameter Unit.
#[derive(Debug)]
pub enum Unit {
    // Input only:
    Document(UnitDocument),
    // Input or output:
    File(UnitFile),
    // Commands:
    Check,
    Concat,
    Copy,
    Merge(UnitCommand),
    Pack,
    Unpack,
    Render(UnitCommand),
    Transform(UnitCommand),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod file_format {
        use super::*;

        mod for_path {
            use super::*;

            #[test]
            fn it_create_json() {
                let json = FileFormat::for_path("yo.json");
                assert_eq!(json, FileFormat::Json);
            }

            #[test]
            fn it_create_toml() {
                let toml = FileFormat::for_path("yo.toml");
                assert_eq!(toml, FileFormat::Toml);
            }

            #[test]
            fn it_create_lua() {
                let lua = FileFormat::for_path("yo.lua");
                assert_eq!(lua, FileFormat::Lua);
            }

            #[test]
            fn it_create_yaml() {
                let yaml = FileFormat::for_path("yo.yaml");
                assert_eq!(yaml, FileFormat::Yaml);
            }

            #[test]
            fn it_create_txt() {
                let txt = FileFormat::for_path("yo.txt");
                assert_eq!(txt, FileFormat::Plain);
            }
        }

        mod for_str {
            use super::*;

            #[test]
            fn it_create_plain() {
                let plain = FileFormat::for_str("plain");
                assert_eq!(plain, Ok(FileFormat::Plain));
            }

            #[test]
            fn it_create_json() {
                let json = FileFormat::for_str("json");
                assert_eq!(json, Ok(FileFormat::Json));
            }

            #[test]
            fn it_create_lua() {
                let lua = FileFormat::for_str("lua");
                assert_eq!(lua, Ok(FileFormat::Lua));
            }

            #[test]
            fn it_create_toml() {
                let toml = FileFormat::for_str("toml");
                assert_eq!(toml, Ok(FileFormat::Toml));
            }

            #[test]
            fn it_create_yaml() {
                let yaml = FileFormat::for_str("yaml");
                assert_eq!(yaml, Ok(FileFormat::Yaml));
            }

            #[test]
            fn it_does_not_create_foo() {
                let foo = FileFormat::for_str("foo");
                assert_eq!(foo, Err(()));
            }
        }
    }
}
