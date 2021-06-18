#![crate_name = "bbee"]

mod javac;
mod bbee_reader;
mod compile;

use structopt::StructOpt;
use std::env;
use walkdir::WalkDir;
use std::io::{Error, ErrorKind};

#[derive(StructOpt)]
#[structopt(about = "a buzzy build tool for the JVM.")]
enum BeeCLI {
    /// Initiailizes a BBee project.
    Init,
    /// Builds an existing BBee project into a jar.
    Build,
    /// Removes existing build data.
    Clean
}

fn main() -> Result<(), Error> {
    let current_path_buf = env::current_dir()?;
    let current_path = current_path_buf.as_path();

    match BeeCLI::from_args() {
        BeeCLI::Build => {
            println!("Building...");

            if !bbee_reader::exists(current_path) {
                println!("Config file does not exist!");
                return Err(Error::new(ErrorKind::NotFound, "Config file not found!"));
            }

            for entry in WalkDir::new(current_path.join("main").join("src")) {

                let ref_entry = &entry?;

                if ref_entry.file_type().is_dir() {
                    continue;
                }
                
                javac::compile(
                    &current_path.join("build").join("classes").as_path(),
                    &ref_entry.path()
                )?;
            }

            compile::compile(current_path)?;

            println!("Finished building!")
        }

        BeeCLI::Init => {
            println!("Initializing Project...");
        }

        BeeCLI::Clean => {

        }
    }

    Ok(())
}
