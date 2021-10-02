use std::error::Error;
use std::result::Result;

/// Generic Result to handle any sort of Error
pub type GenericResult<T> = Result<T, Box<dyn Error>>;
