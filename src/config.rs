use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;

#[derive(Deserialize, Clone)]
pub struct Config {
	pub account: Account,
	pub paths: BTreeMap<String, Path>,
}

#[derive(Deserialize, Clone)]
pub struct Account {
	pub url: String,
	#[serde(default = "default_path_prefix")]
	pub path_prefix: String,
	pub user: String,
	pub password: String,
}

fn default_path_prefix() -> String {
	"/".to_string()
}

#[derive(Deserialize, Clone)]
pub struct Path {
	pub path: String,
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
