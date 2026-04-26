#![allow(special_module_name)]
#![allow(missing_docs)]
mod cmd;
mod config;
mod error;
mod lib;
mod types;
use clap::Parser;
use error::Result;
use types::commands::Cli;
fn main() -> Result<()> {
    config::ensure_data_dir()?;
    let cli = Cli::parse();
    cmd::handle_command(cli.command)
}
