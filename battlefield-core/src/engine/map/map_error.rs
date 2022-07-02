use std::fmt::{self, Display};

#[derive(Debug)]
pub enum MapError {
    FailedToLoad(crate::Error),
}

impl std::error::Error for MapError {}

impl Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FailedToLoad(error) => {
                write!(f, "Failed to load: {error}")
            }
        }
    }
}
