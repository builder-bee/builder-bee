use crate::jar::compile;
use anyhow::Result;
use bbee_config::reader::Config;
use colored::Colorize;
use console::Term;
use spinner::SpinnerBuilder;
use std::time::Instant;

pub fn build(config: &Config) -> Result<()> {
	// Run the compilation
	crate::subcommands::classes::classes(config)?;

	// Benchmark how long it takes to build the jar
	let now = Instant::now();

	// Fancy building spinner
	let spinner = SpinnerBuilder::new(format!(
		"Building {} v{}...",
		config.toml_config.info.name.white(),
		config.toml_config.info.version.white()
	))
	.spinner(crate::bbee_spinner!())
	.start();

	// Finally, compile the jar
	compile::compile(&config.directory, &config.toml_config)?;

	// Stop fancy spinner
	spinner.close();

	Term::stdout().clear_line()?;

	println!(
		"Build {}! ({}s).",
		"successful".green(),
		crate::time::readable_time_elapsed(&now).white()
	);

	Ok(())
}
