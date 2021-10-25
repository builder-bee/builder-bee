use thiserror::Error;
use colored::Colorize;

/// Represents an error that should be given if the config file is not found
#[derive(Debug, Clone, Error)]
#[error("Can not find bbee.toml in {project_directory_name}. Try {} if you want to create a bbee project.", "init".green())]
pub struct ConfigNotFoundError {
	pub project_directory_name: String,
}
