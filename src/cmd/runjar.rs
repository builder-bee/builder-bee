use super::run::run;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;
use expect_macro::expect;

#[derive(Debug, Clone)]
pub struct JavaRunError {
	pub output: String,
}

impl Display for JavaRunError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Javac error: {}", self.output) // user-facing output
	}
}

impl Error for JavaRunError {}

/// Runs a jar file
pub fn javarun(file: &Path) -> Result<String, Box<JavaRunError>> {

	let command_output = expect!(
		run(Command::new("java")
			.arg("-jar")
			.arg(file.display().to_string())),
			"An unknown error occured during parsing the java command."
		);

	if command_output.status.success() {
		Ok(command_output.stdout)
	} else {
		Err(Box::new(JavaRunError {
			output: command_output.stderr,
		}))
	}

}
