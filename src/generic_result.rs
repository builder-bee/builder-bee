use std::error::Error;
use std::result::Result;

pub type GenericResult<T> = Result<T, Box<dyn Error>>;