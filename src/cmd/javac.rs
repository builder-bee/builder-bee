use super::run::run;
use anyhow::{anyhow,Result,Context};
use thiserror::Error;
use std::path::Path;
use std::process::Command;

#[derive(Error, Debug, Clone)]
pub enum JavaCompileError {
	#[error("Could not run javac, file: {file}, target: {target}")]
	CanNotRun {
		file: String,
		target: String
	},
	#[error("Javac error: {output}")]
	Failed {
		output: String
	}
}

/// Compiles a `file` with javac and puts it at the `target` path.
/// For example, given `/build/classes` and `/main/src/HelloWorld.java`,
/// It will generate a file at `/build/classes/HelloWorld.class`
/// With the resulting compiled class.
pub fn compile(target: &Path, file: &Path) -> Result<(), JavaCompileError> {
	let command_output = run(Command::new("javac")
		.arg(file.display().to_string())
		.arg("-d")
		.arg(target.display().to_string()))
		.map_err(|_| JavaCompileError::CanNotRun {
			file: file.display().to_string(),
			target: target.display().to_string()
		})?;

	if command_output.status.success() {
		Ok(())
	} else {
		Err(anyhow!(JavaCompileError::Failed {
			output: command_output.stderr,
		}))
	}
}
