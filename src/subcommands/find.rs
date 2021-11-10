use bbee_config::bbee_reader;
use colored::Colorize;
use std::path::Path;

pub fn find(working_directory: &Path) {
	match bbee_reader::find_config(working_directory) {
		Some(value) => println!("Config: {}", value.display().to_string().green()),
		None => println!(
			"No config found. Use {} to create one",
			"bbee init".yellow()
		),
	};
}
