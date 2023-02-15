use clap::{Parser, Subcommand};

use config::Config;
use lint::Lint;
use run::Run;

pub mod config;
pub mod lint;
pub mod run;

#[derive(Parser)]
#[command(name = "al")]
#[command(version = "0.0")]
#[command(about = "Assembly Line build agent")]
#[command(
    long_about = "Assembly line is a CI/CD system for building and deploying applications focused on using containers as steps on environments that support containers."
)]
#[command(author = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Define a finite list of valid sub commands
#[derive(Subcommand)]
pub enum Commands {
    /// Show/set config
    #[clap(name = "config")]
    Config(Config),
    /// Check a pipeline configuration file for correctness
    Lint(Lint),
    /// Execute the pipeline specified
    Run(Run),
    /// Run as a server, depends on config as to what type - local or true build agent
    Server,
}
