use std::path::Path;
use toml::Value;
use std::error::Error;

pub fn read(path: &Path) -> Result<(), Box<dyn Error>> {

	let file = File::open(path)?;

	let content = file.read_to_string(String::new())?;

}