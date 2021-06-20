use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;
use std::io::{Error, ErrorKind};
use crate::generic_result::GenericResult;

/// Runs a jar file
pub fn run(file: &Path) -> GenericResult<()> {
	
	let status: ExitStatus = Command::new("java")
			.arg("-jar")
			.arg(file.display().to_string())
			.status()?;

	if !status.success() {
		return Err(Box::new(Error::new(ErrorKind::NotFound, "Command line tool java not found")))
	}

	Ok(())

}