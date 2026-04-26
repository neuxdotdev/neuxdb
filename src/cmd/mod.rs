use crate::error::Result;
use crate::types::commands::Commands;
pub mod global;
pub mod job;
pub fn handle_command(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Create { table, columns } => job::create::execute(table, columns),
        Commands::Insert { table, values } => job::insert::execute(table, values),
        Commands::Select {
            table,
            columns,
            r#where,
        } => job::select::execute(table, columns, r#where),
        Commands::Update {
            table,
            set,
            r#where,
        } => job::update::execute(table, set, r#where),
        Commands::Delete { table, r#where } => job::delete::execute(table, r#where),
        Commands::Run { file } => job::run::execute(file),
    }
}
