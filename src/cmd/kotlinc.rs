use super::run::run;
use anyhow::Result;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KotlinCompileError {
	#[error("Could not run kotlinc, file: {file}, target: {target}, error: {error}")]
	CanNotRun { file: String, target: String, error: anyhow::Error },
	#[error("kotlinc error: {output}")]
	Failed { output: String },
}

/// Compiles a `file` with kotlinc and puts it at the `target` path.
/// For example, given `/build/classes` and `/main/src/HelloWorld.kt`,
/// It will generate a file at `/build/classes/HelloWorld.class`
/// With the resulting compiled class.
pub fn compile(target: &Path, file: &Path) -> Result<(), KotlinCompileError> {
	let command_output = run(Command::new("kotlinc")
		.arg(file.display().to_string())
		.arg("-d")
		.arg(target.display().to_string()))
	.map_err(|e| KotlinCompileError::CanNotRun {
		file: file.display().to_string(),
		target: target.display().to_string(),
		error: e
	})?;

	if command_output.status.success() {
		Ok(())
	} else {
		Err(KotlinCompileError::Failed {
			output: command_output.stderr,
		})
	}
}
