use crate::config::bbee_reader::BBeeConfig;
use crate::generic_result::GenericResult;
use crate::jar;
use crate::manifest;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

// Compiles a jar into a working directory with a BBeeConfig.
pub fn compile(working_directory: &Path, config: &BBeeConfig) -> GenericResult<()> {
    let output_file = working_directory.join("build").join("libs");

    fs::create_dir_all(&output_file)
        .expect("Directories could not be created, not enough permissions.");

    let file = File::create(&output_file.join(jar::name::generate(&config)))
        .expect("Jar file could not be created.");

    let mut zip = ZipWriter::new(file);

    let root_class_folder = working_directory.join("build").join("classes");

    for entry in WalkDir::new(&root_class_folder) {
        let ref_entry = &entry?;

        if ref_entry.path() == root_class_folder {
            continue;
        }

        let entry_path: String = ref_entry
            .path()
            .strip_prefix(&root_class_folder)
            .unwrap()
            .display()
            .to_string();

        if ref_entry.file_type().is_dir() {
            zip.add_directory(entry_path, FileOptions::default())?;
            continue;
        }

        zip.start_file(entry_path, FileOptions::default())?;

        zip.write_all(fs::read(ref_entry.path())?.as_slice())?;
    }

    // TODO make sure that the main class is present before writing this.
    zip.add_directory("META-INF", FileOptions::default())?;

    zip.start_file("META-INF/MANIFEST.MF", FileOptions::default())?;

    zip.write_all(manifest::generate(&config)?.as_bytes())?;

    zip.finish()?;

    Ok(())
}
