mod args;
mod cmd;
mod config;

use crate::{
	args::{Opts, SubCommand},
	cmd::{cleanup::cmd as cleanup, cp::cmd as cp, ls::cmd as ls, rm::cmd as rm},
	config::Config,
};
use anyhow::Result;

fn main() -> Result<()> {
	let opts: Opts = args::from_env();
	// read yaml config
	let config = Config::read(&opts.config)?;
	match &opts.subcmd {
		SubCommand::Cp(args) => cp(&config, args, &opts),
		SubCommand::Ls(args) => ls(&config, args, &opts),
		SubCommand::Rm(args) => rm(&config, args, &opts),
		SubCommand::Cleanup(args) => cleanup(&config, args, &opts),
	}
}
