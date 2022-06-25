use juniper::{DefaultScalarValue, GraphQLScalar, InputValue, ScalarValue, Value};
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
    v: &InputValue<S>,
) -> Result<Json<V>, String> {
    Ok(Json(
        serde_json::from_value(input_to_serde_json(v)?).unwrap(),
    ))
}

fn input_to_serde_json<S: ScalarValue>(v: &InputValue<S>) -> Result<serde_json::Value, String> {
    match v {
        InputValue::Null => Ok(serde_json::Value::Null),
        InputValue::Scalar(scalar) => match scalar.clone().into_another() {
            DefaultScalarValue::Boolean(value) => Ok(serde_json::Value::from(value)),
            DefaultScalarValue::Int(value) => Ok(serde_json::Value::from(value)),
            DefaultScalarValue::String(value) => Ok(serde_json::Value::from(value)),
            DefaultScalarValue::Float(value) => Ok(serde_json::Value::from(value)),
        },
        InputValue::List(list) => Ok(serde_json::Value::Array(
            list.iter()
                .map(|span| input_to_serde_json(&span.item))
                .collect::<Result<_, String>>()?,
        )),
        InputValue::Object(object) => Ok(serde_json::Value::Object(
            object
                .iter()
                .map(|(key_span, value_span)| {
                    Ok((
                        key_span.item.clone(),
                        input_to_serde_json(&value_span.item)?,
                    ))
                })
                .collect::<Result<_, String>>()?,
        )),
        InputValue::Variable(..) => Err("Json scalars may not contain variables".to_owned()),
        InputValue::Enum(..) => Err("Json scalars may not contain enums".to_owned()),
    }
}
