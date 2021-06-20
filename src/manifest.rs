use crate::bbee_reader::BBeeConfig;
use crate::generic_result::GenericResult;

pub fn generate_manifest(config: &BBeeConfig) -> GenericResult<String> {

	Ok(format!(r#"Manifest-Version: 1.0
Main-Class: {}
Multi-Release: true"#, config.info.main.as_ref().unwrap_or(&"Main".to_string())))

}