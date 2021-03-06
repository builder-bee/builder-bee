use crate::cmd::runjar;
use crate::jar;
use crate::subcommands::build;
use crate::spinner::DELETE_LINE_CODE;
use anyhow::{anyhow, Result};
use bbee_config::typing::Config;
use colored::Colorize;
use std::time::Instant;

/// Runs the generated jar.
pub fn run_project(config: &Config) -> Result<()> {
	build::build(&config)?;

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

	print!("{}", DELETE_LINE_CODE);

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
