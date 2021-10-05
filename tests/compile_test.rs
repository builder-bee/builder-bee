#[cfg(test)]

mod compile_test {

	use bbee::subcommands::build;
	use relative_path::RelativePath;
	use bbee::generic_result::GenericResult;

	#[test]
	fn assure_compile_success() -> GenericResult<()> {

		build::build(RelativePath::new("./examples/hello_world").to_logical_path("./").as_path())?;

		Ok(())
	}
}