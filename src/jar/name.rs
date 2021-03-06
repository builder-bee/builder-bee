use bbee_config::typing::BBeeConfig;

/// Generates a jar name from a `BBeeConfig`
#[must_use]
pub fn generate(config: &BBeeConfig) -> String {
	format!("{}-{}.jar", config.info.name, config.info.version)
}
