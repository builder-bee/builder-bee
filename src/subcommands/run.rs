use crate::bbee_reader;
use crate::cmd::javarun;
use crate::generic_result::GenericResult;
use crate::jar;
use crate::subcommands::build;
use std::path::Path;

/// Runs the generated jar.
pub fn run(working_directory: &Path) -> GenericResult<()> {
    if !bbee_reader::exists(working_directory) {
        panic!("Config file not found!");
    }

    // Read the config file
    let config = bbee_reader::read(working_directory)?;

    build::build(working_directory)?;

    let jar = working_directory
        .join("build")
        .join("libs")
        .join(jar::name::generate(&config));

    javarun::javarun(&jar)?;

    Ok(())
}
