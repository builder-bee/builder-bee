use crate::compilation::compile::{compile, JavaCompileError};
use crate::config::bbee_reader;
use crate::jar::compile;
use anyhow::{anyhow, Result};
use colored::Colorize;
use spinners::{Spinner, Spinners};
use std::path::Path;
use std::time::Instant;
use thiserror::Error;

/// Represents an error that should be given if the config file is not found
#[derive(Debug, Clone, Error)]
#[error("\nAn error has occured while compiling class {class_file_name}.\n{compile_error_output}\nBuild {}.", "failed".red())]
pub struct JavaBuildError {
	pub class_file_name: String,
	pub compile_error_output: String,
}

pub fn build(working_directory: &Path) -> Result<()> {
	// Read the config file
	let config = bbee_reader::find_and_read(working_directory)?;

	// Fancy building spinner
	let spinner = Spinner::new(
		Spinners::Line,
		format!(
			"Building {} -- v{}...",
			config.toml_config.info.name.white(),
			config.toml_config.info.version.white()
		),
	);

	// Benchmark how long it takes to build the jar
	let now = Instant::now();

	// Run the compilation
	match compile(&config) {
		Ok(_) => (),
		Err(err) => {
			spinner.stop();

			match err {
				JavaCompileError::Entry(err) => return Err(anyhow!(err)),
				JavaCompileError::IO(err) => return Err(anyhow!(err)),

				JavaCompileError::BadCommandCall {
					class_file_name,
					compile_error_output,
				} => {
					return Err(anyhow!(JavaBuildError {
						class_file_name,
						compile_error_output
					}))
				}
			}
		}
	};

	// Finally, compile the jar
	compile::compile(&config.directory, &config.toml_config)?;

	// Stop fancy spinner
	spinner.stop();

	print!("\r\r");

	println!(
		"\nBuild {}! (Took {} seconds)",
		"successful".green(),
		(now.elapsed().as_millis() as f64 / 1000.0)
			.to_string()
			.white()
	);

	Ok(())
}
