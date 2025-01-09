use clap::Parser;
use crate::Engine;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
pub struct Server {
    /// The address of the server
    #[arg(long, default_value_t = String::from("127.0.0.1:4004"))]
    pub addr: String,

    /// The underlying engine to use
    #[arg(long, default_value_t)]
    pub engine: Engine,
}