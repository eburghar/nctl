use anyhow::{Context, Result};
use regex::Regex;
use serde::{de, Deserialize, Deserializer};
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
	#[serde(default = "default_prefix")]
	pub prefix: String,
	pub user: String,
	pub password: String,
}

fn default_prefix() -> String {
	"/".to_string()
}

#[derive(Deserialize, Clone)]
pub struct Path {
	pub base: String,
	pub cleanup: Option<BTreeMap<String, Cleanup>>,
}

#[derive(Deserialize, Clone)]
pub struct Cleanup {
	#[serde(deserialize_with = "regex_deser")]
	pub regex: Regex,
	pub keep: usize,
}

fn regex_deser<'de, D: Deserializer<'de>>(d: D) -> Result<Regex, D::Error> {
	let s: String = Deserialize::deserialize(d)?;
	Regex::new(&s).map_err(de::Error::custom)
}

impl Config {
	pub fn read(config: &str) -> Result<Config> {
		// open configuration file
		let file = File::open(config).with_context(|| format!("Can't open {}", &config))?;
		// deserialize configuration
		let config: Config =
			serde_yaml::from_reader(file).with_context(|| format!("Can't read {}", &config))?;
		Ok(config)
	}
}
