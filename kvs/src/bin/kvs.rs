use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Set the value of a string key to a string
    Set { key: String, value: String },
    /// Get the string value of a given string key
    Get { key: String },
    /// Remove a given key
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
