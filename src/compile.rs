use std::path::Path;
use std::io::Error;
use walkdir::WalkDir;
use std::fs::File;
use zip::ZipWriter;
use std::io::Write;
use std::fs;
use crate::bbee_reader;

pub fn compile(working_directory: &Path, config: bbee_reader::BBeeConfig) -> Result<(), Error> {
	let output_file = working_directory.join("build").join("libs");

	fs::create_dir_all(&output_file).expect("Directories could not be created, not enough permissions.");

	let file = File::create(&output_file.join(format!("{}.jar", config.info.name)))
		.expect("Jar file could not be created.");

	let mut zip = ZipWriter::new(file);

	let root_class_folder = working_directory.join("build").join("classes");

	for entry in WalkDir::new(&root_class_folder) {

		let ref_entry = &entry?;

		if ref_entry.path() == root_class_folder {
			continue;
		}

		let entry_path: String = ref_entry
			.path().display().to_string() // Absolute path
			.chars().skip( // Skip
				root_class_folder.display().to_string().chars().count() // X root chars
			).collect(); // and collect

		if ref_entry.file_type().is_dir() {
			zip.add_directory(entry_path, Default::default())?;
			continue;
		}

		zip.start_file(entry_path, Default::default())?;

		zip.write_all(fs::read(ref_entry.path())?.as_slice())?;
	}
	
	zip.finish()?;

	Ok(())
}