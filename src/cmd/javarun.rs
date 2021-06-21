use std::path::Path;
use std::process::Command;
use crate::generic_result::GenericResult;
use super::run::run;

/// Runs a jar file
pub fn javarun(file: &Path) -> GenericResult<()> {
	
	let commandOutput = run(Command::new("java")
			.arg("-jar")
			.arg(file.display().to_string()))
			.expect("An unknown error occured during parsing the java command.");
	
	Ok(())

}