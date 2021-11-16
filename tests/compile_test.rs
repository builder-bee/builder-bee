#[cfg(test)]

mod compile_test {

	use anyhow::Result;
	use bbee::subcommands::build;
	use bbee_config::reader::find_and_read;
	use std::path::Path;

	#[test]
	fn assure_compile_success() -> Result<()> {
		build::build(
			&find_and_read(Path::new("examples/hello-world"))?
		)?;

		Ok(())
	}
}
