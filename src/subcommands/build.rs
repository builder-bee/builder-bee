use crate::config::bbee_reader;
use crate::cmd::javac;
use crate::generic_result::GenericResult;
use crate::jar::compile;
use colored::*;
use spinners::{Spinner, Spinners};
use std::fs;
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;
use std::fmt;
use std::fmt::Display;
use std::error::Error;

/// Represents an error that should be given if the config file is not found
#[derive(Debug, Clone)]
pub struct JavaBuildError {
	pub class_file_name: String,
	pub compile_error_output: String
}

impl Display for JavaBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\n")?;
		write!(
			f,
			"An error occured while compiling class {}.",
			self.class_file_name
		)?;

		write!(f, "\n")?;
		write!(f, "{}", self.compile_error_output)?;

		write!(
			f,
			"\nBuild {}.",
			"failed".red()
		)?;

		Ok(())
    }
}

impl Error for JavaBuildError {}

pub fn build(working_directory: &Path) -> GenericResult<()> {
    // Read the config file
    let config = bbee_reader::find_and_read(working_directory)?;

	// Fancy building spinner
    let spinner = Spinner::new(
        Spinners::Line,
        format!(
            "Building {} -- v{}...",
            config.toml_config.info.name.white(),
            config.toml_config.info.version.white()
        ),
	);
	
	// Benchmark how long it takes to build the jar
	let now = Instant::now();

    // Walk through all the currentl .java files
    for entry in WalkDir::new(config.directory.join("main").join("src")) {
        // Get a reference of the entry
        let ref_entry = &entry?;

        // Ignore directories
        if ref_entry.file_type().is_dir() {
            continue;
        }

        fs::create_dir_all(&config.directory.join("build").join("classes").as_path())?;

        // Compile it with the javac command line.
        match javac::compile(
            &config.directory.join("build").join("classes").as_path(),
            &ref_entry.path(),
        ) {
            Ok(_) => true,
            Err(error) => {
				spinner.stop();

                return Err(Box::new(JavaBuildError {
					class_file_name: ref_entry.path().display().to_string(),
					compile_error_output: error.output
				}))
            }
        };
    }

    // Finally, compile the jar
    compile::compile(&config.directory, &config.toml_config)?;

	// Stop fancy spinner
    spinner.stop();

	print!("\r\r");

    println!(
        "\nBuild {}! (Took {} seconds)",
        "successful".green(),
        (now.elapsed().as_millis() as f64 / 1000.0)
            .to_string()
            .white()
    );

    Ok(())
}
