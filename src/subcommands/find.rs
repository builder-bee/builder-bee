use crate::config::bbee_reader;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn find(working_directory: &Path) -> Result<()> {
	match bbee_reader::find_config(working_directory) {
		Some(value) => println!(
			"Config: {}",
			value
				.to_str()
				.expect("Could not print out config (not UTF-8?)")
				.green()
		),
		None => println!("No config found."),
	};

	Ok(())
}
