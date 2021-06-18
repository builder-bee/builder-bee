use structopt::StructOpt;
use std::env;

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
        }

        BeeCLI::Init => {
            println!("Initializing Project...");
        }
    }
}
