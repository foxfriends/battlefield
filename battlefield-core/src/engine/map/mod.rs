use crate::data;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Map {
    #[allow(dead_code)]
    path: PathBuf,
    data: Option<data::Map>,
    errors: Vec<crate::Error>,
}

impl Map {
    pub(crate) fn from_file(path: PathBuf) -> Self {
        let data = std::fs::read_to_string(path.join("map.toml"))
            .map_err(Into::into)
            .and_then(|src| toml::from_str(&src).map_err(Into::into));
        let (data, errors) = match data {
            Ok(data) => (Some(data), vec![]),
            Err(error) => (None, vec![error]),
        };
        Self { path, data, errors }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn is_valid(&self) -> bool {
        self.data.is_some() && self.errors.is_empty()
    }

    pub fn name(&self) -> Option<&str> {
        self.data.as_ref().map(|data| data.name.as_str())
    }

    pub fn data(&self) -> Option<&data::Map> {
        self.data.as_ref()
    }

    pub fn errors(&self) -> &[crate::Error] {
        &self.errors
    }
}
