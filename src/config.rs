use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub hosts: BTreeMap<String, Auth>,
}

#[derive(Deserialize, Clone)]
pub struct Auth {
    pub user: String,
    pub password: String,
}

impl Config {
    pub fn read(config: &str) -> Result<Config> {
        // open configuration file
        let file = File::open(&config).with_context(|| format!("Can't open {}", &config))?;
        // deserialize configuration
        let config: Config =
            serde_yaml::from_reader(file).with_context(|| format!("Can't read {}", &config))?;
        Ok(config)
    }
}
