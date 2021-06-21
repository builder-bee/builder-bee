#![crate_name = "bbee"]

mod bbee_reader;
mod cmd;
mod generic_result;
mod jar;
mod manifest;
mod subcommands;

use crate::generic_result::GenericResult;
use crate::subcommands::*;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "a buzzy build tool for the JVM.")]
enum BeeCLI {
    /// Initiailizes a BBee project.
    Init,
    /// Builds an existing BBee project into a jar.
    Build,
    /// Removes existing build data.
    Clean,
    /// Tests the code
    Test,
    /// Runs the built jar in the code
    Run,
}

fn main() -> GenericResult<()> {
    let current_path_buf = env::current_dir().expect("Can not access current working directory!");
    let current_path = current_path_buf.as_path();

    match BeeCLI::from_args() {
        BeeCLI::Build => build::build(current_path),

        BeeCLI::Init => init::init(current_path),

        BeeCLI::Clean => clean::clean(current_path),

        BeeCLI::Test => test::test(current_path),

        BeeCLI::Run => run::run(current_path),
    }?;

    Ok(())
}
