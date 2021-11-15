use anyhow::Result;
use bbee_config::reader::Config;
use std::fs;
use thiserror::Error;

/// An error that occurs when running the clean subcommand
#[derive(Error, Debug)]
pub enum CleanError {
	#[error("Could not remove directory build in {0}: {1}")]
	CouldNotRemoveDirectory(String, std::io::Error),

	#[error("Could not get parent of file {0}.")]
	CouldNotGetParentOfFile(String),
}

/// Remove the build directory from the `working_directory`
pub fn clean(config: &Config) -> Result<(), CleanError> {

	let directory = &config.directory;

	fs::remove_dir_all(
		directory
			.parent()
			.ok_or_else(|| CleanError::CouldNotGetParentOfFile(directory.display().to_string()))?
			.join("build"),
	)
	.map_err(|e| {
		CleanError::CouldNotRemoveDirectory(directory.join("build").display().to_string(), e)
	})?;

	Ok(())
}
