use clap::Parser;

use cli::*;
mod cli;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        Commands::Server => println!("RUN AS A SERVER (not yet implemented)"),
        Commands::Run(run_args) => run::run_command(run_args),
        Commands::Lint(lint_args) => lint::lint_command(lint_args),
        Commands::Config(c) => config::config_commands(c),
    }
}
