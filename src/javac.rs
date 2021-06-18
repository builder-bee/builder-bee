use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;
use std::io::{Error, ErrorKind};


pub fn compile(target: &PathBuf, file: &PathBuf) -> Result<(), std::io::Error> {
	
	let status: ExitStatus = Command::new("javac")
			.arg(file.display().to_string())
			.arg("-d")
			.arg(target.display().to_string())
			.status()?;

	if !status.success() {
		return Err(Error::new(ErrorKind::NotFound, "Command line tool javac not found"))
	}

	Ok(())

}