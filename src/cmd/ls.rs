use crate::{
	args::{Ls, Opts},
	config::Config,
};
use anyhow::{bail, Result};
use chrono::{DateTime, FixedOffset};
use reqwest::{blocking::Client, Method};
use serde::{de, Deserialize, Deserializer};
use std::io::Read;

#[derive(Debug, Deserialize)]
#[serde(rename = "multistatus")]
/// This is the main structure to deserialise a PROPFIND reply from
/// the webdav server. Some tags are renamed for seek of clarity.
pub struct Entries {
	#[serde(rename = "response")]
	pub entries: Vec<Entry>,
}

impl Entries {
	/// sort entries by lastmodified time (most recent first)
	fn sort(&mut self) {
		self.entries.sort_by(|a, b| {
			b.metadata
				.prop
				.lastmodified
				.partial_cmp(&a.metadata.prop.lastmodified)
				.unwrap()
		})
	}
}

#[derive(Debug, Deserialize)]
#[serde(rename = "response")]
pub struct Entry {
	pub href: String,
	#[serde(rename = "propstat")]
	pub metadata: MetaData,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "propstat")]
pub struct MetaData {
	pub prop: Prop,
	#[serde(deserialize_with = "status_deser")]
	pub status: Status,
}

#[derive(Debug)]
pub enum Status {
	Ok,
	Other(u32, String),
}

#[derive(Debug, Deserialize)]
pub struct Prop {
	#[serde(rename = "getlastmodified", deserialize_with = "datetime2822_deser")]
	pub lastmodified: DateTime<FixedOffset>,
	#[serde(deserialize_with = "resourcetype_deser")]
	pub resourcetype: RType,
	#[serde(rename = "getcontentlength")]
	pub contentlength: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceType {
	pub collection: Option<()>,
}

#[derive(Debug, PartialEq)]
pub enum RType {
	Default,
	Collection,
}

/// Convert a rfc2822 date String to a DateTime
fn datetime2822_deser<'de, D: Deserializer<'de>>(d: D) -> Result<DateTime<FixedOffset>, D::Error> {
	let s = Deserialize::deserialize(d)?;
	DateTime::<FixedOffset>::parse_from_rfc2822(s).map_err(de::Error::custom)
}

/// Simplify deserialization of ResourceType to easily check if it's a file or
/// a collection (dir)
fn resourcetype_deser<'de, D: Deserializer<'de>>(d: D) -> Result<RType, D::Error> {
	let t: ResourceType = Deserialize::deserialize(d)?;
	if t.collection.is_some() {
		Ok(RType::Collection)
	} else {
		Ok(RType::Default)
	}
}

/// Convert a String status code to an enum
fn status_deser<'de, D: Deserializer<'de>>(d: D) -> Result<Status, D::Error> {
	let s: String = Deserialize::deserialize(d)?;
	let ss: Vec<&str> = s.split(' ').collect();
	if ss.len() != 3 {
		Err(de::Error::custom(
			"Status should be : HTTP/VERSION CODE MSG",
		))
	} else if ss[2] != "OK" {
		Ok(Status::Other(
			ss[1].parse().map_err(de::Error::custom)?,
			ss[2].to_owned(),
		))
	} else {
		Ok(Status::Ok)
	}
}

/// Get all files from an absolute path
pub fn get_files(conf: &Config, path: &str) -> Result<Entries> {
	let url = format!("{}{}", &conf.account.url, &path);
	let body = r#"<?xml version="1.0" encoding="utf-8" ?>
            <d:propfind xmlns:d="DAV:">
                <d:allprop/>
            </d:propfind>
        "#;
	let mut res = Client::new()
		.request(Method::from_bytes(b"PROPFIND").unwrap(), url)
		.basic_auth(&conf.account.user, Some(&conf.account.password))
		.header("Depth", "Infinity")
		.body(body)
		.send()?;

	if res.status().is_success() {
		let mut buffer = String::new();
		res.read_to_string(&mut buffer)?;
		// println!("{}", buffer);

		let mut entries: Entries = quick_xml::de::from_str(&buffer)?;
		entries.sort();
		Ok(entries)
	} else {
		bail!("Error during ls - {:?}", res)
	}
}

/// Resolve a remote path:file to an absolute path
pub fn resolve_path(conf: &Config, path: &str) -> Result<String> {
	let file: Vec<&str> = path.split(':').collect();
	if file.len() != 2 {
		bail!("argument should be path:[dir]")
	}
	match conf.paths.get(file[0]) {
		Some(alias) => {
			// if the specified file is absolute return as is
			if file[1].starts_with('/') {
				Ok(file[1].to_owned())
			} else {
				// if the base path is absolute prepend to the file
				if alias.base.starts_with('/') {
					Ok(format!("{}/{}", alias.base, file[1]))
				// otherwise prepend the prefix and the base to file
				} else {
					Ok(format!(
						"{}/{}/{}",
						&conf.account.prefix, alias.base, file[1]
					))
				}
			}
		}
		None => bail!("path {} has not been defined", file[0]),
	}
}

pub fn cmd(conf: &Config, args: &Ls, _opts: &Opts) -> Result<()> {
	let abs_path = resolve_path(conf, &args.path)?;
	let entries = get_files(conf, &abs_path)?;
	for entry in entries.entries {
		let name = &entry.href[abs_path.len()..];
		if name.is_empty() {
			println!("./");
		} else {
			println!(
				"./{}{}",
				name,
				if entry.metadata.prop.resourcetype == RType::Collection {
					"/"
				} else {
					""
				}
			);
		}
	}
	Ok(())
}
