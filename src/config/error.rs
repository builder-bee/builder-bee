use thiserror::Error;

/// Represents an error that should be given if the config file is not found
#[derive(Debug, Clone, Error)]
#[error("Can not find bbee.toml in {project_directory_name}")]
pub struct ConfigNotFoundError {
	pub project_directory_name: String,
}
