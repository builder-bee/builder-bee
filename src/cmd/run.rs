use anyhow::Result;
use std::process::Command;
use std::process::ExitStatus;
use std::str;

pub struct CommandOutput {
	pub status: ExitStatus,
	pub stdout: String,
	pub stderr: String,
}

/// Runs a command, spawning the command and waiting for output.
pub fn run(command: &mut Command) -> Result<CommandOutput> {
	let output = command.spawn()?.wait_with_output()?;

	Ok(CommandOutput {
		status: output.status,
		stdout: str::from_utf8(&output.stdout)?.to_owned(),
		stderr: str::from_utf8(&output.stderr)?.to_owned(),
	})
}
