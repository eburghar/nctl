use argh::FromArgs;

#[derive(FromArgs)]
/// Extract latest projects archives from a gitlab server
pub struct Opts {
	#[argh(option, short = 'c', default = "\"/etc/nctl.yml\".to_string()")]
	/// configuration file containing projects and gitlab connection parameters
	pub config: String,
	#[argh(switch, short = 'v')]
	/// more detailed output
	pub verbose: bool,
	#[argh(subcommand)]
	pub subcmd: SubCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum SubCommand {
	Cp(Cp),
	Ls(Ls),
}

#[derive(FromArgs)]
/// Copy a file to/from a webdav folder
#[argh(subcommand, name = "cp")]
pub struct Cp {
	#[argh(positional)]
	/// source
	pub src: String,
	#[argh(positional)]
	/// destination
	pub dst: String,
}

#[derive(FromArgs)]
/// List a webdav folder content
#[argh(subcommand, name = "ls")]
pub struct Ls {
	/// directory to list
	#[argh(positional)]
	pub dir: String,
}
