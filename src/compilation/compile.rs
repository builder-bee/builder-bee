use crate::cmd::javac;
use crate::cmd::kotlinc;
use anyhow::Result;
use std::ffi::OsStr;
use std::fs;
use thiserror::Error;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum CompileError {
	#[error(transparent)]
	Entry(#[from] walkdir::Error),

	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error(
		"An error has occured while compiling class {class_file_name}: {compile_error_output}"
	)]
	BadCommandCall {
		class_file_name: String,
		compile_error_output: String,
	},
	#[error("Could not get extension for file {file_name}")]
	CouldNotGetExtension { file_name: String },
}

/// Compiles a project based on the BBeeConfig, returning the amount of files compiled.
///
/// * `source` the source of all the files
/// * `target` the target to place the files
pub fn compile(source: PathBuf, target: PathBuf) -> Result<i64, CompileError> {
	let mut amount: i64 = 0;

	// Walk through all the current .java files
	for entry in WalkDir::new(source) {
		// Get a reference of the entry
		let ref_entry = &entry?;

		// Ignore directories
		if ref_entry.file_type().is_dir() {
			continue;
		}

		fs::create_dir_all(&target)?;

		match ref_entry
			.path()
			.extension()
			.and_then(OsStr::to_str)
			.ok_or_else(|| CompileError::CouldNotGetExtension {
				file_name: ref_entry.path().display().to_string(),
			})? {
			// Compile it with kotlinc.
			"kt" => match kotlinc::compile(&target, ref_entry.path()) {
				Ok(_) => (),
				Err(error) => {
					return Err(CompileError::BadCommandCall {
						class_file_name: ref_entry.path().display().to_string(),
						compile_error_output: error.to_string(),
					});
				}
			},
			// Compile it with javac.
			"java" => match javac::compile(&target, ref_entry.path()) {
				Ok(_) => (),
				Err(error) => {
					return Err(CompileError::BadCommandCall {
						class_file_name: ref_entry.path().display().to_string(),
						compile_error_output: error.to_string(),
					});
				}
			},
			_ => (),
		};

		amount += 1;
	}

	Ok(amount)
}
