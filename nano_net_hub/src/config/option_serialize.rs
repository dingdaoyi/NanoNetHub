use serde::Deserialize;
use serde_json::Value;

pub fn deserialize_option_string<'de, D>(de: D) -> Result<Option<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(de)?;

    match value {
        Value::String(s) if !s.is_empty() => Ok(Some(s)),
        _ => Ok(None),
    }
}