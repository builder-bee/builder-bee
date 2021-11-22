use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

/// Config name for bbee -- bbee.toml
pub const FILE_NAME: &str = "bbee.toml";

/// The shade mode the dependency is currently in. Default is all.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Shade {
	None,
	Some { package: String },
	All,
}

/// Represents a dependency inside BBEE.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BBeeConfigDependency {
	pub name: String,
	pub version: String,
	pub shade: Shade,
}

impl Display for BBeeConfigDependency {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"{}: v{} (shade: {:?})",
			self.name, self.version, self.shade
		) // user-facing output
	}
}

/// Information about the project
#[derive(Debug)]
pub struct BBeeConfigInfo {
	pub name: String,
	pub main: String,
	pub version: String,
}

/// Per-project/submodule configuration for `BBee`
#[derive(Debug)]
pub struct BBeeConfig {
	pub info: BBeeConfigInfo,
	pub dependencies: Vec<BBeeConfigDependency>,
}

/// Represents a Config -- what directory the config is in and the actual configuration
pub struct Config {
	pub toml_config: BBeeConfig,
	pub directory: PathBuf,
}
