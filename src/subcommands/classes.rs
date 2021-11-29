use crate::compilation::compile::compile;
use crate::bbee_spinner;
use crate::spinner::{DELETE_LINE_CODE, spinner_message};
use anyhow::{anyhow, Result};
use bbee_config::typing::Config;
use colored::Colorize;
use spinner::SpinnerBuilder;
use std::time::Instant;

pub fn classes(config: &Config) -> Result<()> {
	// Fancy class spinner
	let spinner = SpinnerBuilder::new(spinner_message("Compiling", config))
	.spinner(bbee_spinner!())
	.start();

	// Benchmark how long it takes to compile the jar
	let now = Instant::now();

	// Run the compilation
	let amount = match compile(
		config.directory.join("src"),
		config.directory.join("build").join("classes")
	) {
		Ok(value) => value,
		Err(err) => {
			spinner.close();
			println!(); // prints a new line
			return Err(anyhow!(err));
		}
	};

	spinner.close();

	print!("{}", DELETE_LINE_CODE);

	println!(
		"Compilation {}! {} file{} compiled. ({}s).",
		"successful".green(),
		amount.to_string().blue(),
		if amount == 1 { "" } else { "s" }, // 1 file, 2 files
		crate::time::readable_time_elapsed(&now).white()
	);

	Ok(())
}
