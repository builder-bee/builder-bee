use crate::cmd::runjar;
use crate::config::bbee_reader;
use crate::jar;
use crate::subcommands::build;
use anyhow::{anyhow, Result};
use colored::Colorize;
use console::Term;
use std::path::Path;
use std::time::Instant;

/// Runs the generated jar.
pub fn run_project(working_directory: &Path) -> Result<()> {
	// Access this stdout
	let term = Term::stdout();

	// Read the config file
	let config = bbee_reader::find_and_read(working_directory)?;

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

	term.clear_line()?;

	println!(
		"Run {}! (Took {} seconds)",
		"successful".green(),
		(now.elapsed().as_millis() as f64 / 1000.0)
			.to_string()
			.white()
	);

	Ok(())
}

// pub fn run_file(file: &Path) -> Result<()> {

// 	Ok(())

// }
