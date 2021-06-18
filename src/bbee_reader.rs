use std::path::Path;
use std::error::Error;
use std::fs;

static FILE_NAME: &str = "bbee.toml";

/// Reads the bbee file.
pub fn read(working_directory: &Path) -> Result<String, Box<dyn Error>> {

	let config_path = working_directory.join(FILE_NAME);

	let content = fs::read_to_string(config_path)?;

	return Ok(content);

}

/// Check if the bbee config is in the project
pub fn exists(working_directory: &Path) -> bool {
	return working_directory.join(FILE_NAME).exists()
}