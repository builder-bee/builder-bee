#![crate_name = "bbee"]

mod javac;
mod bbee_reader;
mod compile;
mod generic_result;
mod subcommands;
mod manifest;

use structopt::StructOpt;
use std::env;
use crate::subcommands::*;
use crate::generic_result::GenericResult;

#[derive(StructOpt)]
#[structopt(about = "a buzzy build tool for the JVM.")]
enum BeeCLI {
	/// Initiailizes a BBee project.
	Init,
	/// Builds an existing BBee project into a jar.
	Build,
	/// Removes existing build data.
	Clean,
	/// Tests the code
	Test
}

fn main() -> GenericResult<()> {
	let current_path_buf = env::current_dir().expect("Can not access current working directory!");
	let current_path = current_path_buf.as_path();

	match BeeCLI::from_args() {
		BeeCLI::Build => build::build(current_path),

		BeeCLI::Init => init::init(current_path),

		BeeCLI::Clean => clean::clean(current_path),

		BeeCLI::Test => test::test(current_path)
	}?;

	Ok(())
}
