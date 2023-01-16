use crate::{
	args::{Cleanup, Opts},
	cmd::{
		ls::{get_files, resolve_path, RType},
		rm::rm,
	},
	Config,
};
use anyhow::{bail, Result};

pub fn cmd(
	conf: &Config,
	args: &Cleanup,
	Opts {
		verbose, dry_run, ..
	}: &Opts,
) -> Result<()> {
	for path in &args.paths {
		let parts: Vec<&str> = path.split(':').collect();
		if parts.len() != 2 {
			bail!("argument should be path:[cleanup_config]")
		}
		let root_path = format!("{}:", parts[0]);
		let abs_path = resolve_path(conf, &root_path)?;
		// get all files
		let entries = get_files(conf, &abs_path)?;
		// get cleanup config
		match conf.paths[parts[0]].cleanup {
			Some(ref cleanup) => match cleanup.get(parts[1]) {
				Some(cleanup) => {
					if *verbose {
						println!("keep {} file from {}", cleanup.keep, path);
					}
					// get all files matching the regex and skip the nth more recent
					for entry in entries
						.entries
						.into_iter()
						.filter(|entry| {
							entry.metadata.prop.resourcetype == RType::Default
								&& cleanup.regex.is_match(&entry.href[abs_path.len()..])
						})
						.skip(cleanup.keep)
					{
						if *dry_run {
							println!("{} will be removed", &entry.href[abs_path.len()..]);
						} else if rm(conf, &entry.href).is_ok() {
							println!("{} removed", &entry.href[abs_path.len()..]);
						} else {
							eprintln!("failed to remove {}", &entry.href[abs_path.len()..])
						}
					}
				}
				None => bail!(
					"cleanup config '{}' has not been defined for '{}' path",
					parts[1],
					parts[0]
				),
			},
			None => bail!("no cleanup config has been defined for '{}' path", parts[0]),
		}
	}
	Ok(())
}
