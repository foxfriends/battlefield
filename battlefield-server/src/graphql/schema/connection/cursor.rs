use juniper::{InputValue, ParseScalarResult, ParseScalarValue, ScalarToken, ScalarValue, Value};

#[derive(
    Clone, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize, juniper::GraphQLScalar,
)]
pub enum Cursor {
    End,
    Start,
    Node(String),
}

impl Cursor {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(serde_json::to_string(self).unwrap())
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        let string = v
            .as_string_value()
            .ok_or_else(|| format!("Expected `String`, found: {v}"))?;
        serde_json::from_str(string).map_err(|_| "Unrecognized value for `Cursor`".to_owned())
    }

    fn parse_token<S: ScalarValue>(value: ScalarToken<'_>) -> ParseScalarResult<'_, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}

impl From<&str> for Cursor {
    fn from(string: &str) -> Cursor {
        serde_json::from_str(string).unwrap_or(Cursor::Start)
    }
}
