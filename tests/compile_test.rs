#[cfg(test)]

mod compile_test {

	use bbee::subcommands::build;
	use relative_path::RelativePath;
	use anyhow::Result;

	#[test]
	fn assure_compile_success() -> Result<()> {

		build::build(RelativePath::new("./examples/hello_world").to_logical_path("./").as_path())?;

		Ok(())
	}
}