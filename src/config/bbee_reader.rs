use crate::config::error::ConfigNotFoundError;
use anyhow::{anyhow, Context, Result};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::option::Option;
use std::path::{Path, PathBuf};
use toml::Value;

static FILE_NAME: &str = "bbee.toml";

/// Represents a dependency inside BBEE
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BBeeConfigDependency {
	pub name: String,
	pub version: String,
	pub shade: String, // TODO enum?
}

impl Display for BBeeConfigDependency {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"{}: v{} (shade: {})",
			self.name, self.version, self.shade
		) // user-facing output
	}
}

/// Information about the project
#[derive(Debug)]
pub struct BBeeConfigInfo {
	pub name: String,
	pub main: Option<String>,
	pub version: String,
}

/// Per-project/submodule configuration for `BBee`
#[derive(Debug)]
pub struct BBeeConfig {
	pub info: BBeeConfigInfo,
	pub dependencies: Vec<BBeeConfigDependency>,
}

pub struct Config {
	pub toml_config: BBeeConfig,
	pub directory: PathBuf,
}

/// Reads the bbee file and outputs a conf gstruct
pub fn find_and_read(working_directory: &Path) -> Result<Config> {
	let config = find_config(working_directory);

	if config == Option::None {
		return Err(anyhow!(ConfigNotFoundError {
			project_directory_name: working_directory
				.to_str()
				.context("Could not get the working directory")?
				.to_string()
		}));
	};

	let config = config.context("Could not get config.")?;

	return Ok(Config {
		toml_config: read(config.as_path())?,
		directory: config
			.parent()
			.context("Could not get config")?
			.to_path_buf(),
	});
}

#[must_use]
pub fn read(config: &Path) -> Result<BBeeConfig> {
	// Read it using serde's serialization and TOML
	let config_toml = &fs::read_to_string(config)?.parse::<Value>()?;

	// Generate the config object from the Value
	let config = config_from_value(config_toml);

	// Return the config!
	Ok(config)
}

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
					current_directory.to_str().unwrap()
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
fn config_from_value(value: &Value) -> BBeeConfig {
	let info = value.get("info").unwrap(); // Should always be in a bbee config.
	let dependencies = value.get("dependencies");

	return BBeeConfig {
		info: BBeeConfigInfo {
			// Unwrap manditory Name
			name: info.get("name").unwrap().as_str().unwrap().to_string(),

			// Get optional main class.
			main: if info.get("main").is_some() {
				Some(info.get("main").unwrap().as_str().unwrap().to_string())
			} else {
				None
			},

			// Get semi-mandatory version number (defaults to 1.0.0)
			version: info
				.get("version")
				.unwrap_or(&Value::String("1.0.0".to_string()))
				.as_str()
				.unwrap_or("1.0.0")
				.to_string(),
		},

		dependencies: if let Some(value) = dependencies {
			let unwrapped_dependencies = value.as_table().unwrap();

			let mut vector: Vec<BBeeConfigDependency> =
				Vec::with_capacity(unwrapped_dependencies.len());

			// EX "com.google.code.gson:gson" = { version = "2.8.7", shade = "all" }
			for (key, value) in unwrapped_dependencies {
				vector.push(BBeeConfigDependency {
					// Get the name from the key
					name: key.to_string(),

					// Get the version from the table inside
					version: value.get("version").unwrap().as_str().unwrap().to_string(),

					// Shade's default is None
					shade: value
						.get("shade")
						.unwrap_or(&Value::String("none".to_string()))
						.as_str()
						.unwrap()
						.to_string(),
				});
			}

			vector
		} else {
			vec![]
		},
	};
}
