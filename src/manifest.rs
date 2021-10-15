use crate::config::bbee_reader::BBeeConfig;

#[must_use]
pub fn generate(config: &BBeeConfig) -> String {
	format!(
		r#"Manifest-Version: 1.0
Main-Class: {}
Multi-Release: true"#,
		config.info.main.as_ref().unwrap_or(&"Main".to_string())
	)
}
