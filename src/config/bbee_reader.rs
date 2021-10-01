use crate::generic_result::GenericResult;
use crate::config::config_error::ConfigNotFoundError;
use std::fs;
use std::path::Path;
use toml::Value;

static FILE_NAME: &str = "bbee.toml";

/// Represents a dependency inside BBEE
pub struct BBeeConfigDependency {
    pub name: String,
    pub version: String,
    pub shade: String, // TODO enum?
}

/// Information about the project
pub struct BBeeConfigInfo {
    pub name: String,
    pub main: Option<String>,
    pub version: String,
}

/// Per-project/submodule configuration for `BBee`
pub struct BBeeConfig {
    pub info: BBeeConfigInfo,
    pub dependencies: Vec<BBeeConfigDependency>,
}

/// Reads the bbee file and outputs a conf gstruct
pub fn read(working_directory: &Path) -> GenericResult<BBeeConfig> {

	if !exists(working_directory) {
		return Err(Box::new(ConfigNotFoundError {}));
	}

    // Find where the file is
    let config_path = working_directory.join(FILE_NAME);

    // Read it using serde's serialization and TOML
    let config_toml = &fs::read_to_string(config_path)?.parse::<Value>()?;

    // Generate the config object from the Value
    let config = config_from_value(config_toml);

    // Return the config!
    Ok(config)
}

/// Check if the bbee config is in the project
pub fn exists(working_directory: &Path) -> bool {
    working_directory.join(FILE_NAME).exists()
}

/// Get a [BBeeConfig] from a toml Value
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
                        .get("version")
                        .unwrap_or(&Value::String("none".to_string()))
                        .as_str()
                        .unwrap()
                        .to_string(),
                })
            }

            vector
        } else {
            vec![]
        },
    };
}
