use indexmap::IndexMap;
use rlua::{Table as LuaTable, Value as LuaValue};
use serde::{Serialize, Serializer, ser::SerializeMap};
use serde_json::{Value as JsonValue};

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
    let is_object =
            match table.get("_classname") {
                Ok(LuaValue::String(s)) => s.to_str().unwrap() == "Object",
                _ => false,
            };

    if is_object {
        let mut o = IndexMap::new();
        for pair in table.pairs::<LuaValue, LuaValue>() {
            let (entry_idx, entry) = pair.unwrap();

            // Skip not array entry.
            match entry_idx {
                LuaValue::Integer(_) => {},
                _ => continue,
            } // match

            let entry =
                    match entry {
                        LuaValue::Table(t) => t,
                        _ => panic!("wrong object entry"),
                    };

            let name =
                    match entry.get("name") {
                        Ok(LuaValue::String(s)) => s.to_str().unwrap().to_owned(),
                        _ => panic!("wrong object entry name"),
                    };

            let value =
                    match entry.get("value") {
                        Ok(v) => match v {
                            LuaValue::Nil => Value::Nil,
                            LuaValue::Integer(v) => Value::Integer(v),
                            LuaValue::String(s) => Value::String(s.to_str().unwrap().to_owned()),
                            LuaValue::Table(ref t) => from_lua_table(t.clone()),
                            _ => panic!("wrong field value"),
                        },
                        Err(_) => panic!("wrong object entry value"),
                    };

            o.insert(name, value);
        } // for

        Value::Object(o)
    } else if is_lua_table_array(table.clone()) {
        let mut a = Vec::new();
        for pair in table.pairs::<LuaValue, LuaValue>() {
            let (_, elem) = pair.unwrap();
            match elem {
                LuaValue::Nil => {
                    a.push(Value::Nil);
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
                _ => panic!("wrong field value"),
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
                        _ => panic!("wrong field type"),
                    };
            match v {
                LuaValue::Nil => {
                    o.insert(field_name, Value::Nil);
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
                _ => panic!("wrong field value"),
            }
        } // for

        Value::Object(o)
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
