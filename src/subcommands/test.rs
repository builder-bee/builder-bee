use crate::config::bbee_reader;
use anyhow::Result;
use std::path::Path;
use std::option::Option;
use anyhow::anyhow;

pub fn test(working_directory: &Path) -> Result<()> {
	if bbee_reader::find_config(working_directory) == Option::None {
		return Err(anyhow!("This project has not been initialized"));
	}

	// TODO

	Ok(())
}
