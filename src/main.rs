use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "a buzzy build tool for the JVM.")]
enum BeeCLI {
    Init,
    Build
}

fn main() {
    match BeeCLI::from_args() {
        BeeCLI::Build => {
            println!("Building...");
        }

        BeeCLI::Init => {
            println!("Initializing Project...");
        }
    }
}
