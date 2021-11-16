use crate::jar::compile;
use crate::bbee_spinner;
use crate::spinner::spinner_message;
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
	let spinner = SpinnerBuilder::new(spinner_message("Building", config))
	.spinner(bbee_spinner!())
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
