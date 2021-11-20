#![crate_name = "bbee"]
#![warn(missing_docs)]
#![deny(
	clippy::panic,
	clippy::unwrap_used,
	clippy::lossy_float_literal,
	clippy::str_to_string
)]

//! The main entry point for the bbee command-line tool.

extern crate bbee_config;

mod cmd;
mod compilation;
mod jar;
mod manifest;
mod spinner;
mod subcommands;
mod time;

use anyhow::{Context, Result};
use std::time::Instant;

use bbee_config::reader::find_and_read;
use colored::Colorize;
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
	/// Finds the nearest config bbee.toml config
	Find,
	/// Compiles all the classes
	Classes,
}

/// Entry point for builder bee
fn main() {
	match main_err() {
		Ok(_) => (),
		Err(error) => {
			println!("{}: {}", "error".red().bold(), error);
		}
	};
}

/// Main function that could possibly return an error.
fn main_err() -> Result<()> {
	let current_path_buf =
		env::current_dir().context("Can not access current working directory!")?;

	let current_path = current_path_buf.as_path();

	let instant = Instant::now();

	match BeeCLI::from_args() {
		BeeCLI::Find => subcommands::find::find(current_path),
		BeeCLI::Init => subcommands::init::init(current_path)?,
		BeeCLI::Build => subcommands::build::build(&find_and_read(&current_path)?)?,
		BeeCLI::Clean => subcommands::clean::clean(&find_and_read(&current_path)?)?,
		BeeCLI::Test => subcommands::test::test(&find_and_read(&current_path)?)?,
		BeeCLI::Run => subcommands::run::run_project(&find_and_read(&current_path)?)?,
		BeeCLI::Classes => subcommands::classes::classes(&find_and_read(&current_path)?)?,
	};

	println!(
		"All tasks {}. ({}s).",
		"successful".green(),
		crate::time::readable_time_elapsed(&instant).white()
	);

	Ok(())
}
