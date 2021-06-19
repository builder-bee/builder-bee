use std::path::Path;
use crate::generic_result::GenericResult;
use crate::bbee_reader;

pub fn test(working_directory: &Path) -> GenericResult<()> {

	if !bbee_reader::exists(working_directory) {
		panic!("This project has not been initialized!");
	}

	// TODO

	Ok(())
}