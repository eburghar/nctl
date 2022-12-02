use std::path::Path;

use argh::{FromArgs, TopLevelCommand};

/// Extract latest projects archives from a gitlab server
#[derive(FromArgs)]
pub struct Opts {
	/// configuration file containing projects and gitlab connection parameters
	#[argh(option, short = 'c', default = "\"/etc/nctl.yml\".to_string()")]
	pub config: String,

	/// more detailed output
	#[argh(switch, short = 'v')]
	pub verbose: bool,

	#[argh(subcommand)]
	pub subcmd: SubCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum SubCommand {
	Cp(Cp),
}

#[derive(FromArgs)]
/// Copy a file to/from a webdav folder
#[argh(subcommand, name = "cp")]
pub struct Cp {
	/// source
	#[argh(positional)]
	pub src: String,

	/// destination
	#[argh(positional)]
	pub dst: String,
}

/// copy of argh::from_env to insert command name and version in help text
pub fn from_env<T: TopLevelCommand>() -> T {
	let args: Vec<String> = std::env::args().collect();
	let cmd = Path::new(&args[0])
		.file_name()
		.and_then(|s| s.to_str())
		.unwrap_or(&args[0]);
	let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
	T::from_args(&[cmd], &args_str[1..]).unwrap_or_else(|early_exit| {
		println!("{} {}\n", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
		println!("{}", early_exit.output);
		std::process::exit(match early_exit.status {
			Ok(()) => 0,
			Err(()) => 1,
		})
	})
}
