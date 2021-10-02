use crate::generic_result::GenericResult;
use std::process::Command;
use std::process::ExitStatus;
use std::str;

pub struct CommandOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

pub fn run(command: &mut Command) -> GenericResult<CommandOutput> {
    let output = command.output()?;

    Ok(CommandOutput {
        status: output.status,
        stdout: str::from_utf8(&output.stdout)?.to_string(),
        stderr: str::from_utf8(&output.stderr)?.to_string(),
    })
}
