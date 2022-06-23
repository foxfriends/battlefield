use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(name: String, version: String) -> Self {
        Self(format!("{name}@{version}"))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct InvalidModuleId;

impl FromStr for ModuleId {
    type Err = InvalidModuleId;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.matches('@').count() != 1 {
            return Err(InvalidModuleId);
        }
        Ok(Self(string.to_owned()))
    }
}

impl Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
