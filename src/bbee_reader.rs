use std::path::Path;
use toml::Value;
use std::error::Error;

let file_name = "bbee.toml"

pub fn read(current_directory: &Path) -> Result<(), Box<dyn Error>> {

	let file = File::open(path.join(file_name))?;

	let content = file.read_to_string(String::new())?;

}

pub fn exists(current_directory: &Path) -> bool {
	return current_directory.join(file_name).exists()
}