#[cfg(test)]

mod compile_test {

	use bbee::cmd::javac;
	use std::path::Path;
	use bbee::generic_result::GenericResult;

	#[test]
	fn assure_compile_success() -> GenericResult<()> {

		javac::compile(
			Path::new("../examples/hello-world/main/src/HelloWorld.java"),
			Path::new("./test.test")
		)?;

		assert_eq!(2 + 2, 4);

		Ok(())
	}
}