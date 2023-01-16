use crate::{
	args::{Cp, Opts},
	config::Config,
};

use anyhow::{bail, Result};
use reqwest::{
	blocking::{Body, Client},
	header, Method,
};
use std::{fs::File, path::Path};

// For now only the copy of a local file to a remote folder is supported
pub fn cmd(
	conf: &Config,
	args: &Cp,
	Opts {
		verbose, dry_run, ..
	}: &Opts,
) -> Result<()> {
	let file: Vec<&str> = args.dst.split(':').collect();
	if file.len() != 2 {
		bail!("dst should be path:[dir]")
	}

	// get source filename
	let srcpath = Path::new(&args.src);
	let srcname = srcpath.file_name().unwrap().to_str().unwrap();

	// id dst is empty or ends with /, append the source filename
	let dst = if file[1].is_empty() || file[1].ends_with('/') {
		format!("{}{}", file[1], srcname)
	} else {
		file[1].to_owned()
	};

	let path = match conf.paths.get(file[0]) {
		Some(alias) => {
			// if the specified file is absolute return as is
			if file[1].starts_with('/') {
				file[1].to_owned()
			} else {
				// if the aliased path is absolute prepend aliased path to file
				if alias.base.starts_with('/') {
					format!("{}/{}", alias.base, &dst)
				// otherwise prepend prefix and aliased path to file
				} else {
					format!("{}/{}/{}", &conf.account.prefix, alias.base, &dst)
				}
			}
		}
		None => bail!("path {} has not been defined", file[0]),
	};

	let reader = File::open(srcpath)?;
	let metadata = reader.metadata()?;
	let url = format!("{}{}", &conf.account.url, &path);
	if *dry_run {
		println!("{} -> {}:{}", srcname, file[0], path);
		Ok(())
	} else {
		let res = Client::new()
			.request(Method::PUT, url)
			.basic_auth(&conf.account.user, Some(&conf.account.password))
			.header(header::CONTENT_LENGTH, metadata.len())
			.body(Body::new(reader))
			.send()?;

		if res.status().is_success() {
			if *verbose {
				println!("{} -> {}:{}", srcname, file[0], path);
			}
			Ok(())
		} else {
			bail!("Error during put - {:#?}", res)
		}
	}
}
