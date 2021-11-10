//! Manifest utils for generating a MANIFEST.MF config

use bbee_config::bbee_reader::BBeeConfig;

pub const MANIFEST_FILE_NAME: &str = "META-INF/MANIFEST.MF";
pub const MANIFEST_FOLDER_NAME: &str = "META-INF";

#[must_use]
/// Generate a manifest file.
/// Put at META-INF/MANIFEST.MF
pub fn generate(config: &BBeeConfig) -> String {
	format!(
		r#"Manifest-Version: 1.0
Main-Class: {}
Multi-Release: true"#,
		config.info.main
	)
}
