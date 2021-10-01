use std::fmt;
use std::fmt::Display;
use std::error::Error;

/// Represents an error that should be given if the config file is not found
#[derive(Debug, Clone)]
pub struct ConfigNotFoundError;

impl Display for ConfigNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can not find bbee.toml in project directory")
    }
}

impl Error for ConfigNotFoundError {}