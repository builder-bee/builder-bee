use std::path::Path;
use toml::Value;
use std::error::Error;

pub fn read(current_directory: &Path) -> Result<(), Box<dyn Error>> {

	let file = File::open(path.join("bbee.toml"))?;

	let content = file.read_to_string(String::new())?;

}