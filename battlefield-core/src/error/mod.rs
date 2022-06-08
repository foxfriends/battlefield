use std::fmt::{self, Display};

mod convert;
mod error_impl;
mod error_kind;

use error_impl::ErrorImpl;
pub use error_kind::ErrorKind;

#[derive(Debug)]
pub struct Error(ErrorImpl);

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn kind(&self) -> Option<ErrorKind> {
        match &self.0 {
            ErrorImpl::Internal { kind, .. } => Some(*kind),
            _ => None,
        }
    }

    pub(crate) fn external<T>(error: T) -> Self
    where
        T: std::error::Error + Sync + Send + 'static,
    {
        Self(ErrorImpl::from(error))
    }

    pub(crate) fn internal(kind: ErrorKind, message: impl Display) -> Self {
        Self(ErrorImpl::Internal {
            kind,
            message: message.to_string(),
            cause: None,
        })
    }

    #[allow(dead_code)]
    pub(crate) fn internal_with_cause<E>(kind: ErrorKind, message: impl Display, cause: E) -> Self
    where
        E: std::error::Error + Sync + Send + 'static,
    {
        Self(ErrorImpl::Internal {
            kind,
            message: message.to_string(),
            cause: Some(Box::new(cause)),
        })
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0 {
            ErrorImpl::External(cause) => Some(cause.as_ref()),
            ErrorImpl::Internal {
                cause: Some(cause), ..
            } => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            ErrorImpl::External(cause) => write!(f, "battlefield: {}", cause),
            ErrorImpl::Internal {
                kind,
                message,
                cause: None,
            } => {
                write!(f, "battlefield[{:?}] {}", kind, message)
            }
            ErrorImpl::Internal {
                kind,
                message,
                cause: Some(cause),
            } => {
                write!(f, "battlefield[{:?}] {}: {}", kind, message, cause)
            }
        }
    }
}
