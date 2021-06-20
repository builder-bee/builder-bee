use crate::bbee_reader::BBeeConfig;

pub fn jar_name(config: &BBeeConfig) -> String {

	format!(
		"{}-{}.jar",
		config.info.name,
		config.info.version
	)

}