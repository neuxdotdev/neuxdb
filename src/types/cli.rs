use super::commands::Commands;
use clap::Parser;
#[derive(Parser)]
#[command(
    name = "neuxdb",
    about = "Super simple encrypted database",
    version = "0.2.0"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
