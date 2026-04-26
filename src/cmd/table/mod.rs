mod create;
mod delete;
mod drop;
mod insert;
mod list;
mod select;
mod update;
use crate::error::Result;
use crate::types::commands::TableCommands;
pub fn execute(cmd: TableCommands) -> Result<()> {
    match cmd {
        TableCommands::Create { table, columns } => create::execute(table, columns),
        TableCommands::Insert { table, values } => insert::execute(table, values),
        TableCommands::Select {
            table,
            columns,
            r#where,
        } => select::execute(table, columns, r#where),
        TableCommands::Update {
            table,
            set,
            r#where,
        } => update::execute(table, set, r#where),
        TableCommands::Delete { table, r#where } => delete::execute(table, r#where),
        TableCommands::Drop { table, force } => drop::execute(table, force),
        TableCommands::List => list::execute(),
    }
}
