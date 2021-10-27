use crate::config::bbee_reader::BBeeConfig;

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
