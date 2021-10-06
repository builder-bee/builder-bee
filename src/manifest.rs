use crate::config::bbee_reader::BBeeConfig;
use anyhow::Result;

pub fn generate(config: &BBeeConfig) -> Result<String> {
	Ok(format!(
		r#"Manifest-Version: 1.0
Main-Class: {}
Multi-Release: true"#,
		config.info.main.as_ref().unwrap_or(&"Main".to_string())
	))
}
