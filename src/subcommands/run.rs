use crate::config::bbee_reader;
use crate::cmd::javarun;
use crate::generic_result::GenericResult;
use crate::config::config_error::ConfigNotFoundError;
use crate::jar;
use crate::subcommands::build;
use colored::*;
use std::time::Instant;
use std::path::Path;

/// Runs the generated jar.
pub fn run(working_directory: &Path) -> GenericResult<()> {
    if !bbee_reader::exists(working_directory) {
        return Err(Box::new(ConfigNotFoundError {}));
    }

    // Read the config file
    let config = bbee_reader::read(working_directory)?;

    build::build(working_directory)?;

	let now = Instant::now();

    let jar = working_directory
        .join("build")
        .join("libs")
        .join(jar::name::generate(&config));

    let success = match javarun::javarun(&jar) {
		Ok(log) => {
			println!("{}", log);
			
			true
		},
		Err(log) => {

			println!("{}", log);

			false
		},
	};

	println!(
        "Run {}! (Took {} seconds)",
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
