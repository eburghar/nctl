use crate::{
	args::{Opts, Rm},
	cmd::ls::resolve_path,
	Config,
};
use anyhow::{bail, Result};
use reqwest::{blocking::Client, Method};

pub fn rm(conf: &Config, path: &str) -> Result<()> {
	let url = format!("{}{}", &conf.account.url, path);
	let res = Client::new()
		.request(Method::DELETE, url)
		.basic_auth(&conf.account.user, Some(&conf.account.password))
		.send()?;

	if res.status().is_success() {
		Ok(())
	} else {
		bail!("")
	}
}

pub fn cmd(
	conf: &Config,
	args: &Rm,
	Opts {
		verbose, dry_run, ..
	}: &Opts,
) -> Result<()> {
	let mut err = false;
	for file in args.files.iter() {
		let path = resolve_path(conf, file)?;
		if *dry_run {
			println!("{} will be deleted", file);
		} else if rm(conf, &path).is_ok() {
			if *verbose {
				println!("{} has been deleted", file);
			}
		} else {
			eprintln!("Failed to delete {}", file);
			err = true;
		}
	}

	if err {
		bail!("Fail to delete some files")
	} else {
		Ok(())
	}
}
