use crate::compilation::compile::{compile, CompileError};
use crate::config::bbee_reader;
use anyhow::{anyhow, Result};
use colored::Colorize;
use console::Term;
use spinner::SpinnerBuilder;
use std::path::Path;
use std::time::Instant;

pub fn classes(working_directory: &Path) -> Result<()> {
	// Access this stdout
	let term = Term::stdout();

	// Read the config file
	let config = bbee_reader::find_and_read(working_directory)?;

	// Fancy class spinner
	let spinner = SpinnerBuilder::new(format!(
		"Compiling {} -- v{}...",
		config.toml_config.info.name.white(),
		config.toml_config.info.version.white()
	))
	.spinner(crate::bbee_spinner!())
	.start();

	// Benchmark how long it takes to compile the jar
	let now = Instant::now();

	// Run the compilation
	let amount = match compile(&config) {
		Ok(value) => value,
		Err(err) => {
			spinner.update(format!("Build {}", "failed".red()));
			return Err(anyhow!(err));
		}
	};

	spinner.close();

	term.clear_line()?;

	println!(
		"Compilation {}! {} file{} compiled. (Took {} seconds).",
		"successful".green(),
		amount.to_string().blue(),
		if amount == 1 { "" } else { "s" },
		(now.elapsed().as_millis() as f64 / 1000.0)
			.to_string()
			.white()
	);

	Ok(())
}
