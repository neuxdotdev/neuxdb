use super::commands::Commands;
use clap::Parser;
#[derive(Parser)]
#[command(
    name = "neuxdb",
    about = "Super simple encrypted database",
    version = "0.1.0",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
