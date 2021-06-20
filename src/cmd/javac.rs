use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;
use std::io::{Error, ErrorKind};
use crate::generic_result::GenericResult;

/// Compiles a `file` with javac and puts it at the `target` path.
/// For example, given `/build/classes` and `/main/src/HelloWorld.java`,
/// It will generate a file at `/build/classes/HelloWorld.class`
/// With the resulting compiled class.
pub fn compile(target: &Path, file: &Path) -> GenericResult<()> {
	
	let status: ExitStatus = Command::new("javac")
			.arg(file.display().to_string())
			.arg("-d")
			.arg(target.display().to_string())
			.status()?;

	if !status.success() {
		return Err(Box::new(Error::new(ErrorKind::NotFound, "Command line tool javac not found")))
	}

	Ok(())

}