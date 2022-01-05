use indexmap::IndexMap;
use indexmap::IndexSet;
use rlua::{
    String as LuaString,
    Table as LuaTable,
    Value as LuaValue,
};
use serde::{Serialize, Serializer, ser::SerializeMap};
use serde_json::{Value as JsonValue};
use serde_yaml::{Value as YamlValue};
use toml::{Value as TomlValue};

#[derive(Clone, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    Float(f64),
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
            Value::Float(v) => serializer.serialize_f64(v),
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

// Creates a new Value by merging 2 given Values.
// The depth is meant for Array and Object Values. A negative value indicates an infinite depth.
pub fn merge_values(left: &Value, right: &Value, depth: isize) -> Value {
    if depth == 0 {
        return right.clone();
    }

    let dummy = Value::Nil;
    let depth = depth - 1;

    match left {
        Value::Array(l) => match right {
            Value::Array(r) => {
                // Both Values are Array.
                let mut s = Vec::new();

                let left_len = l.len();
                let right_len = r.len();
                let len =
                        if left_len < right_len {
                            right_len
                        } else {
                            left_len
                        };
                let mut idx = 0;
                while idx < len {
                    let (has_left, left_val) =
                            if idx < left_len {
                                (true, l.get(idx).unwrap())
                            } else {
                                (false, &dummy)
                            };
                    let (has_right, right_val) =
                            if idx < right_len {
                                (true, r.get(idx).unwrap())
                            } else {
                                (false, &dummy)
                            };
                    if has_left && has_right {
                        s.push(merge_values(left_val, right_val, depth))
                    } else if has_left {
                        s.push(left_val.clone());
                    } else if has_right {
                        s.push(right_val.clone());
                    }

                    idx += 1;
                } // while

                Value::Array(s)
            },
            _ => right.clone(),
        },
        Value::Object(l) => match right {
            Value::Object(r) => {
                // Both Values are Object.
                let mut s = IndexMap::new();

                let mut keys = IndexSet::new();
                keys.extend(l.keys());
                keys.extend(r.keys());
                for key in keys.iter() {
                    let (has_left, left_val) =
                            match l.get(*key) {
                                Some(v) => (true, v),
                                None => (false, &dummy),
                            };
                    let (has_right, right_val) =
                            match r.get(*key) {
                                Some(v) => (true, v),
                                None => (false, &dummy),
                            };
                    if has_left && has_right {
                        s.insert((*key).clone(), merge_values(left_val, right_val, depth));
                    } else if has_left {
                        s.insert((*key).clone(), left_val.clone());
                    } else if has_right {
                        s.insert((*key).clone(), right_val.clone());
                    }
                } // fof

                Value::Object(s)
            },
            _ => right.clone(),
        },
        _ => right.clone(),
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
        }
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
                    LuaValue::Number(v) => {
                        a.push(Value::Float(v));
                    },
                    LuaValue::String(s) => {
                        a.push(Value::String(s.to_str().unwrap().to_owned()));
                    },
                    LuaValue::Table(ref t) => {
                        a.push(from_lua_table(t.clone()));
                    },
                    _ => panic!("wrong array element"),
                }
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
                                LuaValue::Number(v) => Value::Float(v),
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
                    LuaValue::Number(v) => {
                        a.push(Value::Float(v));
                    },
                    LuaValue::String(s) => {
                        a.push(Value::String(s.to_str().unwrap().to_owned()));
                    },
                    LuaValue::Table(ref t) => {
                        a.push(from_lua_table(t.clone()));
                    },
                    _ => panic!("wrong array element"),
                }
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
                    LuaValue::Number(v) => {
                        o.insert(field_name, Value::Float(v));
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
        }
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
        Value::Float(f) => {
            format!("{}", f)
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
                sb.push('"');
                sb.push_str(k);
                sb.push('"');
                sb.push(',');
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
        JsonValue::Number(v) => {
            if v.is_f64() {
                Value::Float(v.as_f64().unwrap())
            } else {
                Value::Integer(v.as_i64().unwrap())
            }
        },
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
        YamlValue::Number(v) => {
            if v.is_f64() {
                Value::Float(v.as_f64().unwrap())
            } else {
                Value::Integer(v.as_i64().unwrap())
            }
        },
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

// Converts a given TOML value into an internal Value.
fn from_toml_value(value: &TomlValue) -> Value {
    match value {
        TomlValue::Boolean(v) => Value::Boolean(*v),
        TomlValue::Integer(v) => Value::Integer(*v),
        TomlValue::Float(v) => Value::Float(*v),
        TomlValue::Datetime(v) => Value::String(v.to_string()),
        TomlValue::String(v) => Value::String(v.clone()),
        TomlValue::Array(a) => {
            let mut new_array = Vec::new();
            for v in a {
                new_array.push(from_toml_value(v));
            } // for

            Value::Array(new_array)
        },
        TomlValue::Table(o) => {
            let mut new_obj = IndexMap::new();
            for (k, v) in o {
                new_obj.insert(k.clone(), from_toml_value(v));
            } // for

            Value::Object(new_obj)
        },
    }
}

// Converts a given TOML string representation into an internal Value.
pub fn from_toml_str(content: &str) -> Result<Value, ()> {
    let toml_val =
            match toml::from_str(content) {
                Ok(v) => v,
                Err(_) => return Err(()),
            };

    Ok(from_toml_value(&toml_val))
}

// Reorders object content.
// Puts non-array and non-object elements first, then puts array elements and finally puts object elements.
pub fn fix_toml(value: &Value) -> Value {
    if let Value::Object(o) = value {
        let mut new_object = IndexMap::new();
        let mut array_vals = Vec::new();
        let mut object_vals = Vec::new();
        for (k, v) in o {
            match v {
                Value::Array(_) => {
                    array_vals.push((k.clone(), fix_toml(v)));
                },
                Value::Object(_) => {
                    object_vals.push((k.clone(), fix_toml(v)));
                },
                _ => {
                    new_object.insert(k.clone(), fix_toml(v));
                },
            }
        } // for
        for (k, v) in array_vals {
            new_object.insert(k, v);
        } // for
        for (k, v) in object_vals {
            new_object.insert(k, v);
        } // for

        Value::Object(new_object)
    } else {
        value.clone()
    }
}
