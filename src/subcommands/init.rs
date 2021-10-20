use crate::config::bbee_reader;
use anyhow::{anyhow, Result};
use spinner::menu::{Menu, MenuOption, MenuOptional, MenuType};
use std::option::Option;
use std::path::Path;

pub fn init(working_directory: &Path) -> Result<()> {
	if bbee_reader::find_config(working_directory) != Option::None {
		return Err(anyhow!("This project has already been initialized!"));
	}

	let prompt = Menu::new(vec![MenuOption(
		"Project Name".into(),
		MenuType::Text,
		MenuOptional::Required,
		None,
	)]);

	prompt.display();

	// TODO

	Ok(())
}
