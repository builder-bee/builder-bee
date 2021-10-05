use crate::config::bbee_reader;
use crate::generic_result::GenericResult;
use std::path::Path;
use std::option::Option;

pub fn init(working_directory: &Path) -> GenericResult<()> {
	if bbee_reader::find_config(working_directory) != Option::None {
		panic!("This project has already been initialized!");
	}

	// TODO

	Ok(())
}
