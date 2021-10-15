use crate::config::bbee_reader::BBeeConfig;

#[must_use]
pub fn generate(config: &BBeeConfig) -> String {
	format!("{}-{}.jar", config.info.name, config.info.version)
}
