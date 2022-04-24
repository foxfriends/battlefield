use battlefield_core::Scenario;
use std::path::PathBuf;
use tokio::fs::read_to_string;

#[derive(Clone)]
pub struct Scenarios {
    path: PathBuf,
}

impl Scenarios {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub async fn load(&self, name: &str) -> anyhow::Result<Scenario> {
        let scenario_toml = read_to_string(self.path.join(format!("{name}.toml"))).await?;
        Ok(toml::from_str(&scenario_toml)?)
    }
}
