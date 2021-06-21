use super::run::run;
use crate::generic_result::GenericResult;
use std::path::Path;
use std::process::Command;

/// Runs a jar file
pub fn javarun(file: &Path) -> GenericResult<()> {
    run(Command::new("java")
        .arg("-jar")
        .arg(file.display().to_string()))
    .expect("An unknown error occured during parsing the java command.");

    Ok(())
}
