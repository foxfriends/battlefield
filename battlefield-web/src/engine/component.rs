use serde::de::DeserializeOwned;

pub trait Component {
    type Data: DeserializeOwned;

    const NAME: &'static str;
}
