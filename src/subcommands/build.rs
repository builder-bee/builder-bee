use crate::bbee_reader;
use crate::cmd::javac;
use crate::generic_result::GenericResult;
use crate::jar::compile;
use colored::*;
use spinners::{Spinner, Spinners};
use std::fs;
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

pub fn build(working_directory: &Path) -> GenericResult<()> {
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
        ),
	);
	
	let now = Instant::now();

    let mut success = true;

    // Walk through all the currentl .java files
    for entry in WalkDir::new(working_directory.join("main").join("src")) {
        // Get a reference of the entry
        let ref_entry = &entry?;

        // Ignore directories
        if ref_entry.file_type().is_dir() {
            continue;
        }

        fs::create_dir_all(&working_directory.join("build").join("classes").as_path())?;

        // Compile it with the javac command line.
        success = match javac::compile(
            &working_directory.join("build").join("classes").as_path(),
            &ref_entry.path(),
        ) {
            Ok(_) => true,
            Err(error) => {
                println!("\n");
                println!(
                    "An unknown error occured while compiling class {}.",
                    &ref_entry.path().display().to_string()
                );
                println!("\n");
                println!("{}", error.output);

                false
            }
        };
    }

    // Finally, compile the jar
    compile::compile(working_directory, &config)?;

    spinner.stop();

    println!(
        "\nBuild {}! (Took {} seconds)",
        if success {
            "successful".green()
        } else {
            "failed".red()
        },
        (now.elapsed().as_millis() as f64 / 1000.0)
            .to_string()
            .white()
    );

    Ok(())
}
