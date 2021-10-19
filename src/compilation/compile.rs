use std::fs;
use crate::cmd::javac;
use walkdir::WalkDir;
use anyhow::Result;
use thiserror::Error;
use crate::config::bbee_reader::Config;

#[derive(Debug, Error)]
pub enum JavaCompileError {

	#[error(transparent)]
	Entry(#[from] walkdir::Error),

	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error("\nAn error has occured while compiling class {class_file_name}: {compile_error_output}")]
	BadCommandCall {
		class_file_name: String,
		compile_error_output: String
	}
}

pub fn compile(config: &Config) -> Result<(), JavaCompileError> {
	// Walk through all the current .java files
	for entry in WalkDir::new(config.directory.join("main").join("src")) {
		// Get a reference of the entry
		let ref_entry = &entry?;

		// Ignore directories
		if ref_entry.file_type().is_dir() {
			continue;
		}

		fs::create_dir_all(&config.directory.join("build").join("classes").as_path())?;

		// Compile it with the javac command line.
		match javac::compile(
			config.directory.join("build").join("classes").as_path(),
			ref_entry.path(),
		) {
			Ok(_) => true,
			Err(error) => {
				return Err(JavaCompileError::BadCommandCall {
					class_file_name: ref_entry.path().display().to_string(),
					compile_error_output: error.to_string()
				});
			}
		};
	}

	Ok(())
}