mod javac;

use structopt::StructOpt;
use std::env;
use crate::javac::compile;
use walkdir::WalkDir;

#[derive(StructOpt)]
#[structopt(about = "a buzzy build tool for the JVM.")]
enum BeeCLI {
    Init,
    Build
}

fn main() {
    let current_path = env::current_dir().unwrap();

    match BeeCLI::from_args() {
        BeeCLI::Build => {
            println!("Building...");
            for entry in WalkDir::new(current_path.join("main").join("src")) {

                match entry.as_ref() {
                    Ok(v) => {
                        if v.file_type().is_dir() {
                            continue;
                        }
        
                        compile(
                            &current_path.join("target").join("classes"),
                            &v.path().to_path_buf()
                        );
                    }
                    Err(_e) => {
                        continue;
                    }
                }
            }
        }

        BeeCLI::Init => {
            println!("Initializing Project...");
        }
    }
}
