use anyhow::{anyhow, Result};
use std::path::Path;
use colored::Colorize;
use spinner::SpinnerBuilder;
use std::time::Instant;
use crate::config::bbee_reader;
use crate::compilation::compile::{compile, JavaCompileError};

pub fn classes(working_directory: &Path) -> Result<()> {
	// Read the config file
	let config = bbee_reader::find_and_read(working_directory)?;

	// Fancy class spinner
	let spinner = SpinnerBuilder::new(format!(
		"Compiling {} -- v{}...",
		config.toml_config.info.name.white(),
		config.toml_config.info.version.white()
	)).start();

	// Benchmark how long it takes to compile the jar
	let now = Instant::now();

	// Run the compilation
	let amount = match compile(&config) {
		Ok(value) => value,
		Err(err) => {
			spinner.update(format!("Build {}", "failed".red()));

			match err {
				JavaCompileError::Entry(err) => return Err(anyhow!(err)),
				JavaCompileError::IO(err) => return Err(anyhow!(err)),

				JavaCompileError::BadCommandCall {
					class_file_name,
					compile_error_output,
				} => {
					return Err(anyhow!(JavaCompileError::BadCommandCall {
						class_file_name,
						compile_error_output
					}))
				}
			}
		}
	};

	spinner.close();

	println!(
		"\r\r\nCompilation {}! {} file{} compiled. (Took {} seconds).",
		"successful".green(),
		amount.to_string().blue(),
		if amount == 1 { "" } else { "s" },
		(now.elapsed().as_millis() as f64 / 1000.0)
			.to_string()
			.white()
	);

	Ok(())
}