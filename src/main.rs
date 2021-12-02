mod args;
mod cmd;
mod config;
mod utils;

use crate::{
	args::{Opts, SubCommand},
	cmd::cp::cmd as cp,
	config::Config,
};
use anyhow::Result;

fn main() -> Result<()> {
	let opts: Opts = argh::from_env();
	// read yaml config
	let config = Config::read(&opts.config)?;
	match &opts.subcmd {
		SubCommand::Cp(args) => cp(&config, args, &opts),
		SubCommand::Ls(_) => Ok(()),
	}
}
