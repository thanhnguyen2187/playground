use std::ops::AddAssign;
use clap::{Parser, Args};

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    Get(ArgsKey),
    Set(ArgsKeyValue),
    Rm(ArgsKey),
}

#[derive(Args)]
struct ArgsKey {
    key: String,
}

#[derive(Args)]
struct ArgsKeyValue {
    key: String,
    value: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get(_) => panic!("unimplemented"),
        Commands::Set(_) => panic!("unimplemented"),
        Commands::Rm(_) => panic!("unimplemented"),
    }
}
