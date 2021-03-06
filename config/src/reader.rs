use crate::error::ConfigNotFoundError;
use crate::typing::*;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::option::Option;
use std::path::{Path, PathBuf};
use toml::Value;

/// Reads the bbee file and outputs a conf gstruct
pub fn find_and_read(working_directory: &Path) -> Result<Config> {
	let config = match find_config(working_directory) {
		Option::None => {
			return Err(anyhow!(ConfigNotFoundError {
				project_directory_name: working_directory.display().to_string()
			}))
		}
		Option::Some(v) => v,
	};

	return Ok(Config {
		toml_config: read(config.as_path())?,
		directory: config
			.parent()
			.context("Could not get config")?
			.to_path_buf(),
	});
}

/// Reads a config from a path -- usually bbee.toml
pub fn read(config: &Path) -> Result<BBeeConfig> {
	// Read it using serde's serialization and TOML
	let config_toml = &fs::read_to_string(config)?.parse::<Value>()?;

	// Generate the config object from the Value
	config_from_value(config_toml)
}

/// Finds the lcoation of a configuration
#[must_use]
pub fn find_config(current_directory: &Path) -> Option<PathBuf> {
	let buf = current_directory.to_path_buf();

	let mut directory = Path::new(&buf);

	let mut config_file: Option<PathBuf> = grab_in_directory(directory);

	while config_file == Option::None {
		directory = directory
			.parent()
			.with_context(|| {
				format!(
					"Could not find bbee.toml in {} or any of its parents",
					current_directory.display()
				)
			})
			.ok()?;

		config_file = grab_in_directory(directory);
	}

	config_file
}

/// Check if the bbee config is in this directory
#[must_use]
pub fn grab_in_directory(directory: &Path) -> Option<PathBuf> {
	let path = directory.join(FILE_NAME);

	if path.exists() {
		Option::Some(path)
	} else {
		Option::None
	}
}

/// Get a `BBeeConfig` from a toml Value
fn config_from_value(value: &Value) -> Result<BBeeConfig> {
	let info = value
		.get("info")
		.ok_or_else(|| anyhow!("Could not find info table"))?; // Should always be in a bbee config.
	let dependencies = value.get("dependencies");

	return Ok(BBeeConfig {
		info: BBeeConfigInfo {
			// Unwrap manditory Name
			name: info
				.get("name")
				.ok_or_else(|| anyhow!("Name not found"))?
				.as_str()
				.ok_or_else(|| anyhow!("Name is not string!"))?
				.to_owned(),

			// Get optional main class.
			main: info
				.get("main")
				.unwrap_or(&Value::String("1.0.0".to_owned()))
				.as_str()
				.ok_or_else(|| anyhow!("[info] main is not a string"))?
				.to_owned(),

			// Get semi-mandatory version number (defaults to 1.0.0)
			version: info
				.get("version")
				.unwrap_or(&Value::String("1.0.0".to_owned()))
				.as_str()
				.unwrap_or("1.0.0")
				.to_owned(),
		},

		dependencies: if let Some(value) = dependencies {
			let unwrapped_dependencies = value
				.as_table()
				.ok_or_else(|| anyhow!("Dependency is not a table!"))?;

			// EX "com.google.code.gson:gson" = { version = "2.8.7", shade = "all" }
			unwrapped_dependencies
				.iter()
				.map(|(key, value)| -> Result<BBeeConfigDependency> {
					Ok(BBeeConfigDependency {
						// Get the name from the key
						name: key.to_string(),

						// Get the version from the table inside
						version: value
							.get("version")
							.ok_or_else(|| anyhow!("Version not found"))?
							.as_str()
							.ok_or_else(|| anyhow!("Version is not a string"))?
							.to_owned(),

						// Shade's default is None
						shade: match value
							.get("shade")
							.unwrap_or(&Value::String("all".to_owned()))
							.as_str()
							.ok_or_else(|| anyhow!("Shade value is not a string"))?
						{
							"all" => Shade::All,
							"none" => Shade::None,
							_ => return Err(anyhow!("Shade must be either all or none!")),
						},
					})
				})
				.take_while(Result::is_ok)
				.map(Result::unwrap)
				.collect()
		} else {
			vec![]
		},
	});
}
