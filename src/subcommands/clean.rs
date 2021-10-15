use crate::config::bbee_reader;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Remove the build directory from the `working_directory`
pub fn clean(working_directory: &Path) -> Result<()> {
	let config = bbee_reader::find_and_read(working_directory)?;

	fs::remove_dir_all(config.directory.join("build")).with_context(|| {
		format!(
			"Could not remove directory build in {}",
			config
				.directory
				.join("build")
				.to_str()
				.context("Could not remove directory build")
				.unwrap()
		)
	})?;

	Ok(())
}
