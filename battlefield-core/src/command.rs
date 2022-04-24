use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct Command(String);

impl FromStr for Command {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}
