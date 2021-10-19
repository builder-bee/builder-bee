use crate::cmd::runjar;
use crate::config::bbee_reader;
use crate::jar;
use crate::subcommands::build;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;
use std::time::Instant;

/// Runs the generated jar.
pub fn run(working_directory: &Path) -> Result<()> {
	// Read the config file
	let config = bbee_reader::find_and_read(working_directory)?;

	build::build(&config.directory)?;

	let now = Instant::now();

	let jar = &config
		.directory
		.join("build")
		.join("libs")
		.join(jar::name::generate(&config.toml_config));

	let success = match runjar::javarun(jar) {
		Ok(log) => {
			println!("{}", log);
			true
		}
		Err(log) => {
			println!("{}", log);

			false
		}
	};

	println!(
		"Run {}! (Took {} seconds)",
		if success {
			"successful".green()
		} else {
			"failed".red()
		},
		(now.elapsed().as_millis() as f64 / 1000.0)
			.to_string()
			.white()
	);

	Ok(())
}
