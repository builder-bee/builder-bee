use crate::config::bbee_reader;
use crate::generic_result::GenericResult;
use std::path::Path;

pub fn test(working_directory: &Path) -> GenericResult<()> {
    if !bbee_reader::exists(working_directory) {
        panic!("This project has not been initialized!");
    }

    // TODO

    Ok(())
}
