use crate::bbee_reader;
use crate::generic_result::GenericResult;
use std::path::Path;

pub fn init(working_directory: &Path) -> GenericResult<()> {
    if bbee_reader::exists(working_directory) {
        panic!("This project has already been initialized!");
    }

    // TODO

    Ok(())
}
