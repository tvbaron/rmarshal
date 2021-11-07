use indexmap::IndexMap;
use rlua::{
    String as LuaString,
    Table as LuaTable,
    Value as LuaValue,
};
use serde::{Serialize, Serializer, ser::SerializeMap};
use serde_json::{Value as JsonValue};
use serde_yaml::{Value as YamlValue};

#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::Nil => serializer.serialize_none(),
            Value::Boolean(v) => serializer.serialize_bool(v),
            Value::Integer(v) => serializer.serialize_i64(v),
            Value::String(ref v) => serializer.serialize_str(v),
            Value::Array(ref a) => a.serialize(serializer),
            Value::Object(ref o) => {
                let mut s = serializer.serialize_map(Some(o.len()))?;
                for (k, v) in o {
                    s.serialize_entry(k, v)?;
                } // for
                s.end()
            },
        }
    }
}

// Tests whether a given Lua table can be treated as an array.
fn is_lua_table_array(table: LuaTable) -> bool {
    let mut idx = 1;
    for pair in table.pairs::<LuaValue, LuaValue>() {
        let (k, _) = pair.unwrap();
        match k {
            LuaValue::Integer(i) => {
                if i != idx {
                    return false;
                }
            },
            _ => return false,
        } // match
        idx += 1;
    } // for

    true
}

// Converts a given Lua table into an internal Value.
pub fn from_lua_table(table: LuaTable) -> Value {
    let classname =
            match table.get("_classname") {
                Ok(LuaValue::String(s)) => Some(s.to_str().unwrap().to_owned()),
                _ => None,
            };

    if let Some(cname) = classname {
        if cname == "NullClass" {
            Value::Nil
        } else if cname == "Array" {
            let mut a = Vec::new();
            for pair in table.pairs::<LuaValue, LuaValue>() {
                let (_, elem) = pair.unwrap();
                match elem {
                    LuaValue::Nil => {
                        a.push(Value::Nil);
                    },
                    LuaValue::Boolean(v) => {
                        a.push(Value::Boolean(v));
                    },
                    LuaValue::Integer(v) => {
                        a.push(Value::Integer(v));
                    },
                    LuaValue::String(s) => {
                        a.push(Value::String(s.to_str().unwrap().to_owned()));
                    },
                    LuaValue::Table(ref t) => {
                        a.push(from_lua_table(t.clone()));
                    },
                    _ => panic!("wrong array element"),
                } // match
            } // for

            Value::Array(a)
        } else if cname == "Object" {
            let mut o = IndexMap::new();
            let keys: LuaTable =
                    match table.get("keys") {
                        Ok(k) => k,
                        Err(_) => panic!("wrong object (keys)"),
                    };
            let values: LuaTable =
                    match table.get("values") {
                        Ok(k) => k,
                        Err(_) => panic!("wrong object (values)"),
                    };
            for key in keys.sequence_values::<LuaString>() {
                let key =
                        match key {
                            Ok(k) => k,
                            Err(_) => panic!("wrong object (key)"),
                        };
                let value =
                        match values.get(key.clone()) {
                            Ok(v) => match v {
                                LuaValue::Boolean(v) => Value::Boolean(v),
                                LuaValue::Integer(v) => Value::Integer(v),
                                LuaValue::String(s) => Value::String(s.to_str().unwrap().to_owned()),
                                LuaValue::Table(ref t) => from_lua_table(t.clone()),
                                _ => panic!("wrong object value"),
                            },
                            Err(_) => panic!("wrong object (value"),
                        };
                o.insert(key.to_str().unwrap().to_owned(), value);
            } // for

            Value::Object(o)
        } else {
            panic!("wrong classname");
        }
    } else {
        // No classname.
        if is_lua_table_array(table.clone()) {
            let mut a = Vec::new();
            for pair in table.pairs::<LuaValue, LuaValue>() {
                let (_, elem) = pair.unwrap();
                match elem {
                    LuaValue::Nil => {
                        a.push(Value::Nil);
                    },
                    LuaValue::Boolean(v) => {
                        a.push(Value::Boolean(v));
                    },
                    LuaValue::Integer(v) => {
                        a.push(Value::Integer(v));
                    },
                    LuaValue::String(s) => {
                        a.push(Value::String(s.to_str().unwrap().to_owned()));
                    },
                    LuaValue::Table(ref t) => {
                        a.push(from_lua_table(t.clone()));
                    },
                    _ => panic!("wrong array element"),
                } // match
            } // for

            Value::Array(a)
        } else {
            let mut o = IndexMap::new();
            for pair in table.pairs::<LuaValue, LuaValue>() {
                let (k, v) = pair.unwrap();
                let field_name =
                        match k {
                            LuaValue::String(s) => s.to_str().unwrap().to_owned(),
                            _ => panic!("wrong table key"),
                        };
                match v {
                    LuaValue::Nil => {
                        o.insert(field_name, Value::Nil);
                    },
                    LuaValue::Boolean(v) => {
                        o.insert(field_name, Value::Boolean(v));
                    },
                    LuaValue::Integer(v) => {
                        o.insert(field_name, Value::Integer(v));
                    },
                    LuaValue::String(s) => {
                        o.insert(field_name, Value::String(s.to_str().unwrap().to_owned()));
                    },
                    LuaValue::Table(ref t) => {
                        o.insert(field_name, from_lua_table(t.clone()));
                    },
                    _ => panic!("wrong table value"),
                }
            } // for

            Value::Object(o)
        }
    }
}

pub fn from_processed_template(table: LuaTable) -> String {
    let mut res = String::new();
    for pair in table.pairs::<LuaValue, LuaValue>() {
        let (_, elem) = pair.unwrap();
        match elem {
            LuaValue::Boolean(b) => {
                if b {
                    res.push_str("true");
                } else {
                    res.push_str("false");
                }
            },
            LuaValue::Integer(i) => {
                res.push_str(&i.to_string());
            },
            LuaValue::String(s) => {
                res.push_str(s.to_str().unwrap());
            },
            LuaValue::Table(t) => {
                let classname =
                        match t.get("_classname") {
                            Ok(LuaValue::String(s)) => Some(s.to_str().unwrap().to_owned()),
                            _ => None,
                        };

                if let Some(cname) = classname {
                    if cname == "NullClass" {
                        res.push_str("null");
                    } else {
                        panic!("wtf");
                    }
                } else {
                    panic!("wtf");
                }
            },
            _ => panic!("wtf"),
        } // match
    } // for

    res
}

// Converts a given internal value into lua.
pub fn to_lua_string(value: &Value) -> String {
    match value {
        Value::Nil => "NULL".to_owned(),
        Value::Boolean(b) => {
            if *b {
                "true".to_owned()
            } else {
                "false".to_owned()
            }
        },
        Value::Integer(i) => {
            format!("{}", i)
        },
        Value::String(s) => {
            let mut sb = String::new();
            sb.push('"');
            sb.push_str(s);
            sb.push('"');

            sb
        },
        Value::Array(a) => {
            let mut sb = String::new();
            sb.push_str("Array:new({");
            for (_, elem) in a.iter().enumerate() {
                sb.push_str(&to_lua_string(elem));
                sb.push(',');
            } // for
            sb.push_str("})");

            sb
        },
        Value::Object(o) => {
            let mut sb = String::new();
            sb.push_str("Object:new({");
            for (k, v) in o {
                sb.push('{');
                sb.push_str(k);
                sb.push('=');
                sb.push_str(&to_lua_string(v));
                sb.push('}');
                sb.push(',');
            } // for
            sb.push_str("})");

            sb
        },
    }
}

// Converts a given JSON value into an internal Value.
fn from_json_value(value: &JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::Nil,
        JsonValue::Bool(v) => Value::Boolean(*v),
        JsonValue::Number(v) => Value::Integer(v.as_i64().unwrap()),
        JsonValue::String(v) => Value::String(v.clone()),
        JsonValue::Array(a) => {
            let mut new_array = Vec::new();
            for v in a {
                new_array.push(from_json_value(v));
            } // for

            Value::Array(new_array)
        },
        JsonValue::Object(o) => {
            let mut new_obj = IndexMap::new();
            for (k, v) in o {
                new_obj.insert(k.clone(), from_json_value(v));
            } // for

            Value::Object(new_obj)
        },
    }
}

// Converts a given JSON string representation into an internal Value.
pub fn from_json_str(content: &str) -> Result<Value, ()> {
    let json_val =
            match serde_json::from_str(content) {
                Ok(v) => v,
                Err(_) => return Err(()),
            };

    Ok(from_json_value(&json_val))
}

// Converts a given YAML value into an internal Value.
fn from_yaml_value(value: &YamlValue) -> Value {
    match value {
        YamlValue::Null => Value::Nil,
        YamlValue::Bool(v) => Value::Boolean(*v),
        YamlValue::Number(v) => Value::Integer(v.as_i64().unwrap()),
        YamlValue::String(v) => Value::String(v.clone()),
        YamlValue::Sequence(a) => {
            let mut new_array = Vec::new();
            for v in a {
                new_array.push(from_yaml_value(v));
            } // for

            Value::Array(new_array)
        },
        YamlValue::Mapping(o) => {
            let mut new_obj = IndexMap::new();
            for (k, v) in o {
                let name =
                        match k {
                            YamlValue::String(n) => n.clone(),
                            _ => panic!("wrong object entry name"),
                        };

                new_obj.insert(name, from_yaml_value(v));
            } // for

            Value::Object(new_obj)
        },
    }
}

// Converts a given YAML string representation into an internal Value.
pub fn from_yaml_str(content: &str) -> Result<Value, ()> {
    let yaml_val =
            match serde_yaml::from_str(content) {
                Ok(v) => v,
                Err(_) => return Err(()),
            };

    Ok(from_yaml_value(&yaml_val))
}
