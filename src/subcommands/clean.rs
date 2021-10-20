use crate::config::bbee_reader;
use anyhow::{Context, Result};
use thiserror::Error;
use std::fs;
use std::path::Path;

/// An error that occurs when running the clean subcommand
#[derive(Error, Debug)]
pub enum CleanError {
	#[error("No config found")]
	NoConfigFound,

	#[error("Could not remove directory build in {0}: {1}")]
	CouldNotRemoveDirectory(String, std::io::Error)
}

/// Remove the build directory from the `working_directory`
pub fn clean(working_directory: &Path) -> Result<(), CleanError> {
	let directory = bbee_reader::find_config(working_directory)
		.ok_or_else(|| CleanError::NoConfigFound)?;

	fs::remove_dir_all(directory.join("build"))
		.map_err(|e| CleanError::CouldNotRemoveDirectory(
			directory.join("build").to_str().unwrap_or("unknown").to_string(), e
		))?;

	Ok(())
}
