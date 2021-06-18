use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[structopt(about = "a buzzy build tool for the JVM.")]
enum Cli {
    Build
}

fn main() {
    let cli = Cli::from_args();
}
