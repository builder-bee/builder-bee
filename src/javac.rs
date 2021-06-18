use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

pub fn compile(target: &PathBuf, file: &PathBuf) {
	
	let status: ExitStatus = Command::new("javac")
			.arg(file.display().to_string())
			.arg("-d")
			.arg(target.display().to_string())
			.status().unwrap();

	if !status.success() {
		panic!("An unexpected error occured.")
	}

}