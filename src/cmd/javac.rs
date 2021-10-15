use super::run::run;
use anyhow::{Result,Context};
use thiserror::Error;
use std::path::Path;
use std::process::Command;

#[derive(Error, Debug, Clone)]
#[error("Javac error: {output}")]
pub struct JavaCompileError {
	pub output: String,
}

/// Compiles a `file` with javac and puts it at the `target` path.
/// For example, given `/build/classes` and `/main/src/HelloWorld.java`,
/// It will generate a file at `/build/classes/HelloWorld.class`
/// With the resulting compiled class.
pub fn compile(target: &Path, file: &Path) -> Result<()> {
	let command_output = run(Command::new("javac")
		.arg(file.display().to_string())
		.arg("-d")
		.arg(target.display().to_string()))
		.with_context(|| format!(
			"Can not run command javac, file: {}, target: {}",
			file.display().to_string(),
			target.display().to_string()
		))?;

	if command_output.status.success() {
		Ok(())
	} else {
		Err(JavaCompileError {
			output: command_output.stderr,
		})
	}
}
