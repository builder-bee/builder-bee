use crate::config::bbee_reader;
use anyhow::{anyhow, Result};
use std::option::Option;
use std::path::Path;

pub fn init(working_directory: &Path) -> Result<()> {
	if bbee_reader::find_config(working_directory) != Option::None {
		anyhow!("This project has already been initialized!");
	}

	// TODO

	Ok(())
}
