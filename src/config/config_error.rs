use std::fmt;
use std::fmt::Display;
use std::error::Error;

/// Represents an error that should be given if the config file is not found
#[derive(Debug, Clone)]
pub struct ConfigNotFoundError {
	pub project_directory_name: String
}

impl Display for ConfigNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can not find bbee.toml in {}", self.project_directory_name)
    }
}

impl Error for ConfigNotFoundError {}