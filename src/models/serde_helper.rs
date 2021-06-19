use serde::de::{self, Deserialize, Deserializer, Unexpected};
use serde_json::value::Value;

pub fn default_as_false() -> bool {
    false
}

pub fn default_as_zero() -> i64 {
    0
}

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;

    match v.as_bool() {
        Some(b) => Ok(b),
        None => match v.as_i64() {
            Some(c) => Ok(c != 0),
            None => Err(de::Error::invalid_value(Unexpected::Unit, &"zero or one")),
        },
    }
}
