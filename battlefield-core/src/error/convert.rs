macro_rules! error_from {
    ($errortype:ty) => {
        impl From<$errortype> for super::Error {
            fn from(error: $errortype) -> Self {
                super::Error::external(error)
            }
        }
    };
}

error_from!(toml::de::Error);
error_from!(std::io::Error);
