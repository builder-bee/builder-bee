use bbee_config::bbee_reader;
use anyhow::anyhow;
use anyhow::Result;
use std::option::Option;
use std::path::Path;

pub fn test(working_directory: &Path) -> Result<()> {
	if bbee_reader::find_config(working_directory) == Option::None {
		return Err(anyhow!("This project has not been initialized"));
	}

	// TODO

	Ok(())
}
