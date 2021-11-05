use crate::config::bbee_reader;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn find(working_directory: &Path) -> Result<()> {
	match bbee_reader::find_config(working_directory) {
		Some(value) => println!(
			"Config: {}",
			value.display().to_string().green()
		),
		None => println!("No config found. Use {} to create one", "bbee init".yellow()),
	};

	Ok(())
}
