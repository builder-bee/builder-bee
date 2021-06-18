#![crate_name = "bbee"]

mod javac;

use structopt::StructOpt;
use std::env;
use crate::javac::compile;
use walkdir::WalkDir;
use std::error::Error;

#[derive(StructOpt)]
#[structopt(about = "a buzzy build tool for the JVM.")]
enum BeeCLI {
    Init,
    Build
}

fn main() -> Result<(), Box<dyn Error>> {
    let current_path = env::current_dir()?;

    match BeeCLI::from_args() {
        BeeCLI::Build => {
            println!("Building...");
            for entry in WalkDir::new(current_path.join("main").join("src")) {

                let exposed_entry = entry?;

                if exposed_entry.file_type().is_dir() {
                    continue;
                }

                compile(
                    &current_path.join("build").join("classes").as_path(),
                    &exposed_entry.path()
                )?;
            }
        }

        BeeCLI::Init => {
            println!("Initializing Project...");
        }
    }

    Ok(())
}
