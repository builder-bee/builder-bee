#![crate_name = "bbee"]

mod javac;
mod bbee_reader;
mod compile;
mod generic_result;
mod subcommands;

use structopt::StructOpt;
use std::env;
use crate::generic_result::GenericResult;

#[derive(StructOpt)]
#[structopt(about = "a buzzy build tool for the JVM.")]
enum BeeCLI {
	/// Initiailizes a BBee project.
	Init,
	/// Builds an existing BBee project into a jar.
	Build,
	/// Removes existing build data.
	Clean
}

fn main() -> GenericResult<()> {
	let current_path_buf = env::current_dir()?;
	let current_path = current_path_buf.as_path();

	match BeeCLI::from_args() {
		BeeCLI::Build => {
			subcommands::build::build(current_path);
		}

		BeeCLI::Init => {
			println!("Initializing Project...");
		}

		BeeCLI::Clean => {

		}
	}

	Ok(())
}
