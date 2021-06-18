use std::path::Path;
use std::fs;
use crate::generic_result::GenericResult;
use crate::bbee_reader;

pub fn clean(working_directory: &Path) -> GenericResult<()> {

	if !bbee_reader::exists(working_directory) {
		panic!("Config file not found!");
	}

	fs::remove_dir_all(working_directory.join("build"))?;

	Ok(())
}