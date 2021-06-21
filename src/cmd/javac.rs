use std::path::Path;
use std::process::Command;
use crate::generic_result::GenericResult;
use super::run::run;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct JavaCompileError {
	pub output: String
}

impl Display for JavaCompileError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Javac error: {}", self.output) // user-facing output
    }
}

impl Error for JavaCompileError { 
}

/// Compiles a `file` with javac and puts it at the `target` path.
/// For example, given `/build/classes` and `/main/src/HelloWorld.java`,
/// It will generate a file at `/build/classes/HelloWorld.class`
/// With the resulting compiled class.
pub fn compile(target: &Path, file: &Path) -> Result<(), Box<JavaCompileError>> {
	
	let command_output = run(Command::new("javac")
			.arg(file.display().to_string())
			.arg("-d")
			.arg(target.display().to_string()))
			.expect("An unknown error occured during parsing the javac command.");

	if command_output.status.success() {
		Ok(())
	} else {
		Err(Box::new(JavaCompileError { output: command_output.stderr }))
	}

}