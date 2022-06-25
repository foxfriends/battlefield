use juniper::{GraphQLScalar, InputValue, ScalarValue, Value};
use serde::de::{DeserializeOwned, IntoDeserializer};
use serde::{Deserialize, Serialize};

#[derive(GraphQLScalar)]
#[graphql(to_output_with = json_to_output, from_input_with = input_to_json, parse_token(i32, f64, String))]
pub struct Json<S>(pub S)
where
    S: Serialize + for<'de> Deserialize<'de>;

fn json_to_output<V: Serialize + DeserializeOwned, S: ScalarValue>(v: &Json<V>) -> Value<S> {
    serde_json_to_output(serde_json::json! { v.0 })
}

fn serde_json_to_output<S: ScalarValue>(json: serde_json::Value) -> Value<S> {
    match json {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Array(array) => {
            Value::List(array.into_iter().map(serde_json_to_output).collect())
        }
        serde_json::Value::Object(object) => Value::Object(
            object
                .into_iter()
                .map(|(k, v)| (k, serde_json_to_output(v)))
                .collect(),
        ),
        scalar => Value::Scalar(S::deserialize(scalar.into_deserializer()).unwrap()),
    }
}

fn input_to_json<S: ScalarValue, V: Serialize + DeserializeOwned>(
    _v: &InputValue<S>,
) -> Result<Json<V>, String> {
    unimplemented!("Currently not needed, as no query takes input Json")
}
