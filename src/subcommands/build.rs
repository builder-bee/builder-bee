use crate::config::bbee_reader;
use crate::jar::compile;
use anyhow::Result;
use colored::Colorize;
use console::Term;
use spinner::SpinnerBuilder;
use std::path::Path;
use std::time::Instant;

pub fn build(working_directory: &Path) -> Result<()> {
	// Access this stdout
	let term = Term::stdout();

	// Read the config file
	let config = bbee_reader::find_and_read(working_directory)?;

	// Run the compilation
	crate::subcommands::classes::classes(working_directory)?;

	// Benchmark how long it takes to build the jar
	let now = Instant::now();

	// Fancy building spinner
	let spinner = SpinnerBuilder::new(format!(
		"Building {} -- v{}...",
		config.toml_config.info.name.white(),
		config.toml_config.info.version.white()
	))
	.spinner(crate::bbee_spinner!())
	.start();

	// Finally, compile the jar
	compile::compile(&config.directory, &config.toml_config)?;

	// Stop fancy spinner
	spinner.close();

	term.clear_line()?;

	println!(
		"Build {}! (Took {} seconds).",
		"successful".green(),
		(now.elapsed().as_millis() as f64 / 1000.0)
			.to_string()
			.white()
	);

	Ok(())
}
