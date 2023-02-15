use clap::Args;
use std::path::PathBuf;

/// Arguments for the lint command to control behaviour
#[derive(Args)]
pub struct Lint {
    /// The path of the pipeline file to check for correctness
    pub path: PathBuf,
}

/// Lint the pipeline specified by the arguments for correctness
pub fn lint_command(args: Lint) {
    println!("YOU CHOSE TO LINT: {:?} (not yet implemented)", args.path)
}
