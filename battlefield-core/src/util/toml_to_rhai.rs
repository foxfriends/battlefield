use rhai::{Array, Dynamic, Map};
use toml::Value;

pub(crate) fn toml_to_rhai(toml: &Value) -> Dynamic {
    match toml {
        Value::Array(array) => array.iter().map(toml_to_rhai).collect::<Array>().into(),
        Value::Boolean(val) => Dynamic::from(*val),
        Value::Datetime(val) => Dynamic::from(val.to_string()),
        Value::Float(val) => Dynamic::from(*val),
        Value::Integer(val) => Dynamic::from(*val),
        Value::String(val) => Dynamic::from(val.clone()),
        Value::Table(val) => val
            .iter()
            .map(|(k, v)| (From::from(k), toml_to_rhai(v)))
            .collect::<Map>()
            .into(),
    }
}
