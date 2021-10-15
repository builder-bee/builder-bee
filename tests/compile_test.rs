#[cfg(test)]

mod compile_test {

	use anyhow::Result;
	use bbee::subcommands::build;
	use relative_path::RelativePath;

	#[test]
	fn assure_compile_success() -> Result<()> {
		build::build(
			RelativePath::new("./examples/hello_world")
				.to_logical_path("./")
				.as_path(),
		)?;

		Ok(())
	}
}
