use clap::Args;
use std::path::PathBuf;

/// Arguments for the run command to control execution
#[derive(Args)]
pub struct Run {
    /// The path of the pipeline file to run locally
    pub path: PathBuf,
}

/// Run the pipeline as specified by the args on the local machine
pub fn run_command(args: Run) {
    println!("YOU CHOSE TO RUN: {:?} (not yet implemened)", args.path)
}
