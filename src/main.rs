#![crate_name = "bbee"]
#![warn(missing_docs)]

//! The main entry point for the bbee command-line tool.

mod cmd;
mod jar;
mod manifest;
mod subcommands;
mod config;
use anyhow::{Context,Result};

use std::env;
use structopt::StructOpt;

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
	Test,
	/// Runs the built jar in the code
	Run,
}

fn main() {
	better_panic::install();

	match main_err() {
		Ok(_) => (),
		Err(error) => {
			println!("error: {}", error);
			()
		}
	};
}

fn main_err() -> Result<()> {
	let current_path_buf = env::current_dir()
		.context("Can not access current working directory!")?;
	
	let current_path = current_path_buf.as_path();

	match BeeCLI::from_args() {
		BeeCLI::Build => subcommands::build::build(current_path),

		BeeCLI::Init => subcommands::init::init(current_path),

		BeeCLI::Clean => subcommands::clean::clean(current_path),

		BeeCLI::Test => subcommands::test::test(current_path),

		BeeCLI::Run => subcommands::run::run(current_path),
	}?;

	Ok(())
}
