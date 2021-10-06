use super::run::run;
use std::path::Path;
use std::process::Command;
use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum JavaRunError {
	#[error("No output was found")]
	OutputNotFound,
	#[error("Javac error: {output}")]
	CommandFailed {
		output: String
	}
}

/// Runs a jar file
pub fn javarun(file: &Path) -> Result<String, JavaRunError> {

	let command = run(Command::new("java")
		.arg("-jar")
		.arg(file.display().to_string()));

	let command_output = match command {
		Ok(value) => value,
		Err(_) => return Err(JavaRunError::OutputNotFound)
	};

	if command_output.status.success() {
		Ok(command_output.stdout)
	} else {
		Err(JavaRunError::CommandFailed { output: command_output.stderr })
	}

}
