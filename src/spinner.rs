//! Spinner utils for builder bee

use bbee_config::reader::Config;
use colored::Colorize;

pub static SPINNER_FORMAT: &[&str] = &["-", "\\", "|", "/"];

/// Return bbee's spinner format.
///
/// This macro turns the static array of spinner strings into a simple `bbee_spinner!()` command.
#[macro_export]
macro_rules! bbee_spinner {
	() => {
		(*crate::spinner::SPINNER_FORMAT).to_vec()
	};
}

pub fn spinner_message(task_name: &str, config: &Config) -> String {
	format!(
		"{} {} v{}...",
		task_name,
		config.toml_config.info.name.white(),
		config.toml_config.info.version.white()
	)
}