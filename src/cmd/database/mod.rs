mod backup;
mod create;
mod export;
mod import;
mod info;
mod restore;
use crate::error::Result;
use crate::types::commands::DatabaseCommands;
pub fn execute(cmd: DatabaseCommands) -> Result<()> {
    match cmd {
        DatabaseCommands::Create { path } => create::execute(path),
        DatabaseCommands::Info => info::execute(),
        DatabaseCommands::Export { output, passphrase } => export::execute(output, passphrase),
        DatabaseCommands::Import { input, passphrase } => import::execute(input, passphrase),
        DatabaseCommands::Backup { name } => backup::execute(name),
        DatabaseCommands::Restore { file } => restore::execute(file),
    }
}
