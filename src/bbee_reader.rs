use std::path::Path;
use std::fs;
use serde::Deserialize;
use crate::generic_result::GenericResult;

static FILE_NAME: &str = "bbee.toml";

#[derive(Deserialize)]
pub struct BBeeConfigDependency {
	pub name: String,
	pub version: String,
	pub r#type: String
}

#[derive(Deserialize)]
pub struct BBeeConfigInfo {
	pub name: String,
	
	#[serde(default = "default_version")]
	pub version: String
}

fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Deserialize)]
pub struct BBeeConfig {
	pub info: BBeeConfigInfo
}

/// Reads the bbee file and outputs a conf gstruct
pub fn read(working_directory: &Path) -> GenericResult<BBeeConfig> {

	// Find where the file is
	let config_path = working_directory.join(FILE_NAME);

	// Read it using serde's serialization and TOML
	let config: BBeeConfig = toml::from_str(&fs::read_to_string(config_path)?)?;

	// Return the config!
	Ok(config)

}

/// Check if the bbee config is in the project
pub fn exists(working_directory: &Path) -> bool {
	working_directory.join(FILE_NAME).exists()
}