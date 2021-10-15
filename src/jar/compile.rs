use crate::config::bbee_reader::BBeeConfig;
use crate::jar;
use crate::manifest;
use anyhow::{Context, Result};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

// Compiles a jar into a working directory with a BBeeConfig.
pub fn compile(working_directory: &Path, config: &BBeeConfig) -> Result<()> {
	let output_file = working_directory.join("build").join("libs");

	fs::create_dir_all(&output_file)
		.context("Directories could not be created, not enough permissions.")?;

	let file = File::create(&output_file.join(jar::name::generate(config)))
		.context("Jar file could not be created.")?;

	let mut zip = ZipWriter::new(file);

	let options = FileOptions::default();

	let root_class_folder = working_directory.join("build").join("classes");

	for entry in WalkDir::new(&root_class_folder) {
		let ref_entry = &entry?;

		if ref_entry.path() == root_class_folder {
			continue;
		}

		let entry_path: String = ref_entry
			.path()
			.strip_prefix(&root_class_folder)?
			.display()
			.to_string();

		if ref_entry.file_type().is_dir() {
			zip.add_directory(entry_path, options)?;
			continue;
		}

		zip.start_file(entry_path, options)
			.context("Could not create entry path")?;

		zip.write_all(&*fs::read(ref_entry.path())?)
			.with_context(|| {
				format!(
					"Could not write file {} to jar.",
					ref_entry.path().to_str().unwrap()
				)
			})?;
	}

	// TODO make sure that the main class is present before writing this.
	zip.add_directory("META-INF", options)?;

	zip.start_file("META-INF/MANIFEST.MF", options)?;

	zip.write_all(manifest::generate(config).as_bytes())?;

	zip.finish().with_context(|| {
		format!(
			"Could not finish writing jar file in {}",
			working_directory.to_str().unwrap()
		)
	})?;

	Ok(())
}
