use crate::config::bbee_reader::BBeeConfig;

pub fn generate(config: &BBeeConfig) -> String {
	format!("{}-{}.jar", config.info.name, config.info.version)
}
