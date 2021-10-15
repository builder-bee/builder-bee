use super::run::run;
use anyhow::Result;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum JavaRunError {
	#[error("No output was found")]
	OutputNotFound,
	#[error("Javac error: {output}")]
	CommandFailed { output: String },
}

/// Runs a jar file
pub fn javarun(file: &Path) -> Result<String, JavaRunError> {
	let command = run(Command::new("java")
		.arg("-jar")
		.arg(file.display().to_string()))
	.map_err(|_| JavaRunError::OutputNotFound)?;

	if command.status.success() {
		Ok(command.stdout)
	} else {
		Err(JavaRunError::CommandFailed {
			output: command.stderr,
		})
	}
}
