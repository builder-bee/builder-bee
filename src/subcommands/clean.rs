use crate::config::bbee_reader;
use crate::generic_result::GenericResult;
use std::fs;
use std::path::Path;

/// Remove the build directory from the `working_directory`
pub fn clean(working_directory: &Path) -> GenericResult<()> {

	let config = bbee_reader::find_and_read(working_directory)?;

    fs::remove_dir_all(config.directory.join("build"))?;

    Ok(())
}
