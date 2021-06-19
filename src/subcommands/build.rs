use walkdir::WalkDir;
use std::path::Path;
use crate::generic_result::GenericResult;
use crate::bbee_reader;
use crate::javac;
use crate::compile;
use std::time::Instant;
use colored::*;
use spinners::{Spinner, Spinners};

pub fn build(working_directory: &Path) -> GenericResult<()> {

	let now = Instant::now();

	// Need to make sure the config file is here
	if !bbee_reader::exists(working_directory) {
		panic!("Config file not found!");
	}
	
	// Read the config file
	let config = bbee_reader::read(working_directory)?;

	let spinner = Spinner::new(
		Spinners::Line,
		format!(
			"Building {} -- v{}...",
			config.info.name.white(),
			config.info.version.white()
		)
	);

	// Walk through all the currentl .java files
	for entry in WalkDir::new(working_directory.join("main").join("src")) {

		// Get a reference of the entry
		let ref_entry = &entry?;

		// Ignore directories
		if ref_entry.file_type().is_dir() {
			continue;
		}
		
		// Compile it with the javac command line.
		javac::compile(
			&working_directory.join("build").join("classes").as_path(),
			&ref_entry.path()
		)?;
	}

	// Finally, compile the jar
	compile::compile(working_directory, config)?;

	spinner.stop();

	println!(
		"\nBuild {}! (Took {} seconds)",
		"successful".green(),
		(now.elapsed().as_millis() as f32 / 1000.0).to_string().white()
	);

	Ok(())
}