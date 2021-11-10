use bbee_config::reader;
use colored::Colorize;
use std::path::Path;

/// Finds a config in a directory.
/// 
/// * `directory` The directory to start searching in.anyhow
/// 
/// Calls find_config, which will recursively check the parent for any bbee.toml file.
pub fn find(directory: &Path) {
	match reader::find_config(directory) {
		Some(value) => println!("Config: {}", value.display().to_string().green()),
		None => println!(
			"No config found. Use {} to create one",
			"bbee init".yellow()
		),
	};
}
