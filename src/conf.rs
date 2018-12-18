
use scl::Value as SclValue;
use serde_json::{self, Value};
use serde::de::DeserializeOwned;
use std::path::Path;

pub struct Conf;

impl Conf {
    pub fn load_file<P, T>(path: P) -> T
        where P: AsRef<Path>,
              T: DeserializeOwned
    {
        let config = match scl::parse_file(path) {
            Ok(c) => c,
            Err(e) => {
                println!("Error loading config file: {}", e);
                ::std::process::exit(0);
            }
        };

        match serde_json::from_value(Conf::from(SclValue::Dict(config))) {
            Ok(j) => j,
            Err(e) => {
                println!("Error occurred while loading config file: {}", e);
                ::std::process::exit(0);
            }
        }
    }

    fn from(val: SclValue) -> serde_json::Value {
        match val {
            SclValue::Boolean(b) => Value::Bool(b),
            SclValue::Integer(i) => serde_json::to_value(i).unwrap_or_default(),
            SclValue::Float(f) => serde_json::to_value(f).unwrap_or_default(),
            SclValue::String(s) => Value::String(s),
            SclValue::Dict(d) => {
                let mut map = serde_json::Map::new();
                for (k, v) in d.into_iter() {
                    map.insert(k,Conf::from(v)).unwrap_or_default();
                }
                Value::Object(map)
            },
            SclValue::Array(a) => Value::Array(a
                .into_iter()
                .map(|v| Conf::from(v))
                .collect::<Vec<Value>>()),
            SclValue::Date(d) => {
                let mut map = serde_json::Map::new();
                map.insert(String::from("day"), serde_json::to_value(d.day).unwrap_or_default());
                map.insert(String::from("month"), serde_json::to_value(d.month).unwrap_or_default());
                map.insert(String::from("year"), serde_json::to_value(d.year).unwrap_or_default());
                Value::Object(map)
            }
        }

    }
}