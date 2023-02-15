use clap::{Parser, Subcommand};

/// Defines the options available to the `config` sub command,
#[derive(Parser)]
pub struct Config {
    #[command(subcommand)]
    command: ConfigCommand,
}

/// Define a finite list of options that `config`'s `command` can provide
#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Show the current config
    Get,
    /// Generate an example config based on a CRI-API supported runtime
    #[clap(name = "generate")]
    Generate(Generate),
}

/// Defines options available to the `config generate` sub command
#[derive(Parser)]
pub struct Generate {
    #[command(subcommand)]
    command: GenerateConfigCommand,
}

/// Select a CRI-API enabled endpoint to generate default configuration for
#[derive(Subcommand)]
pub enum GenerateConfigCommand {
    /// Generate config that communicates with a default install of Containerd
    Containerd,
    /// Generate config that communicates with a default install of CRI-O
    CriO,
    /// Generate config that communicates with a default install od Docker
    Docker,
}

/// command handler for the config sub command
pub fn config_commands(config: Config) {
    match config.command {
        ConfigCommand::Get => println!("GET CONFIG CALLED"),
        ConfigCommand::Generate(flavour) => generate_template_config(flavour),
    }
}

/// Calls the generator for the selected CRI-API enabled runtime
pub fn generate_template_config(flavour: Generate) {
    match flavour.command {
        GenerateConfigCommand::Containerd => generate_example_containerd_config(),
        GenerateConfigCommand::CriO => generate_example_crio_config(),
        GenerateConfigCommand::Docker => generate_example_docker_config(),
    }
}

/// Generate Containerd specific config based on a default install
pub fn generate_example_containerd_config() {
    println!("THIS IS WHERE WE OUTPUT CONTAINERD SPECIFIC CONFIG")
}

/// Generate CRI-O specific config based on a default install
pub fn generate_example_crio_config() {
    println!("YOU CHOSE CRI-O")
}

/// Generate Docker specific config based on a default install
pub fn generate_example_docker_config() {
    println!("DOCKER FOR THE WIN")
}
