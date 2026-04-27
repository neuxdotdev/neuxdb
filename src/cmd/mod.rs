use crate::error::Result;
use crate::types::commands::TopLevelCommands;
pub mod config;
pub mod database;
pub mod run;
pub mod table;
pub fn handle_command(cmd: TopLevelCommands) -> Result<()> {
    match cmd {
        TopLevelCommands::Database { cmd } => database::execute(cmd),
        TopLevelCommands::Table { cmd } => table::execute(cmd),
        TopLevelCommands::Run { file } => run::execute(file),
        TopLevelCommands::Config { cmd } => config::execute(cmd),
    }
}
