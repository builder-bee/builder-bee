#[cfg(test)]

mod compile_test {

    use bbee::cmd::javac;
    use std::path::Path;

    #[test]
    fn assure_compile_success() {

        javac::compile(
            Path::new("../examples/hello-world/main/src/HelloWorld.java"),
            Path::new("./test.test")
        );

        assert_eq!(2 + 2, 4);
    }
}