use super::ErrorKind;
use std::error::Error;

#[derive(Debug)]
pub(super) enum ErrorImpl {
    External(Box<dyn Error + Sync + Send>),
    Internal {
        kind: ErrorKind,
        message: String,
        cause: Option<Box<dyn Error + Sync + Send>>,
    },
}

impl<T> From<T> for ErrorImpl
where
    T: Error + Sync + Send + 'static,
{
    fn from(error: T) -> Self {
        Self::External(Box::new(error))
    }
}
