use crate::config::bbee_reader;
use crate::generic_result::GenericResult;
use std::fs;
use std::path::Path;

/// Remove the build directory from the `working_directory`
pub fn clean(working_directory: &Path) -> GenericResult<()> {
    if !bbee_reader::exists(working_directory) {
        panic!("Config file not found!");
    }

    fs::remove_dir_all(working_directory.join("build"))?;

    Ok(())
}
