use crate::cmd::runjar;
use crate::jar;
use crate::subcommands::build;
use anyhow::{anyhow, Result};
use bbee_config::reader;
use colored::Colorize;
use console::Term;
use std::path::Path;
use std::time::Instant;

/// Runs the generated jar.
pub fn run_project(working_directory: &Path) -> Result<()> {
	// Read the config file
	let config = reader::find_and_read(working_directory)?;

	build::build(&config.directory)?;

	let now = Instant::now();

	let jar = &config
		.directory
		.join("build")
		.join("libs")
		.join(jar::name::generate(&config.toml_config));

	match runjar::javarun(jar) {
		Ok(log) => {
			println!("{}", log);
		}
		Err(log) => {
			println!("Run {}. Error: {}", "failed".red(), log);
			return Err(anyhow!(log));
		}
	};

	Term::stdout().clear_line()?;

	println!(
		"Run {}! ({}s)",
		"successful".green(),
		crate::time::readable_time_elapsed(&now).white()
	);

	Ok(())
}

// pub fn run_file(file: &Path) -> Result<()> {

// 	Ok(())

// }
