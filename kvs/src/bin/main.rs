use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    Get,
    Set,
    Rm,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get => println!("get"),
        Commands::Set => println!("set"),
        Commands::Rm => println!("rm"),
    }
}
