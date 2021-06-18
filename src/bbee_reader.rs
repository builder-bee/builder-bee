use std::path::Path;
use std::error::Error;
use std::fs;
use serde::Deserialize;


static FILE_NAME: &str = "bbee.toml";

#[derive(Deserialize)]
pub struct BBeeConfigInfo {
	name: String
}

#[derive(Deserialize)]
pub struct BBeeConfig {
	info: BBeeConfigInfo
}

/// Reads the bbee file.
pub fn read(working_directory: &Path) -> Result<BBeeConfig, Box<dyn Error>> {

	let config_path = working_directory.join(FILE_NAME);

	let config: BBeeConfig = toml::from_str(&fs::read_to_string(config_path)?)?;

	return Ok(config)

}

/// Check if the bbee config is in the project
pub fn exists(working_directory: &Path) -> bool {
	return working_directory.join(FILE_NAME).exists()
}