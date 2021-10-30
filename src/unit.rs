const JSON_PATH_SUFFIX: &str = ".json";
const TOML_PATH_SUFFIX: &str = ".toml";
const YAML_PATH_SUFFIX: &str = ".yaml";

const LUA_PATH_SUFFIX: &str = ".yaml";

#[derive(Debug, PartialEq, Eq)]
pub enum FileFormat {
    Unknown,

    Json,
    Toml,
    Yaml,

    Lua,
    // Template,
}

impl Default for FileFormat {
    fn default() -> Self {
        FileFormat::Unknown
    }
}

impl FileFormat {
    pub fn for_path(path: &str) -> Self {
        let lc_path = path.to_lowercase();
        if lc_path.ends_with(JSON_PATH_SUFFIX) {
            FileFormat::Json
        } else if lc_path.ends_with(TOML_PATH_SUFFIX) {
            FileFormat::Toml
        } else if lc_path.ends_with(YAML_PATH_SUFFIX) {
            FileFormat::Yaml
        } else if lc_path.ends_with(LUA_PATH_SUFFIX) {
            FileFormat::Lua
        } else {
            FileFormat::Unknown
        }
    }

    pub fn for_str(format: &str) -> Self {
        match format {
            "json" => FileFormat::Json,
            "toml" => FileFormat::Toml,
            "yaml" => FileFormat::Yaml,
            "lua" => FileFormat::Lua,
            _ => FileFormat::Unknown,
        }
    }

    pub fn is_known(&self) -> bool {
        *self != FileFormat::Unknown
    }
}

#[derive(Debug, Default)]
pub struct UnitFile {
    pub path: String,
    pub format: FileFormat,
    pub pretty: Option<bool>,
}

impl UnitFile {
    pub fn for_path(path: &str) -> Self {
        UnitFile {
            path: path.to_owned(),
            format: FileFormat::for_path(path),
            pretty: None,
        }
    }

    pub fn for_path_format(path: &str, format: FileFormat) -> Self {
        UnitFile {
            path: path.to_owned(),
            format,
            pretty: None,
        }
    }

    pub fn for_format(format: FileFormat) -> Self {
        UnitFile {
            path: String::new(),
            format,
            pretty: None,
        }
    }
}

// Parameter Unit.
#[derive(Debug)]
pub enum Unit {
    File(UnitFile),
    Merge,
}
