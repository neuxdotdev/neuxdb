use super::{
    create_table::create_table, delete_rows::delete_rows, insert_row::insert_row,
    select_rows::select_rows, update_rows::update_rows,
};
use crate::error::{NeuxError, Result};
use crate::lib::compiler::parse;
use crate::lib::sintax::Statement;
pub fn execute_query(sql: &str) -> Result<()> {
    let stmt = parse(sql).map_err(|e| NeuxError::Parse(e.to_string()))?;
    match stmt {
        Statement::CreateTable { name, columns } => create_table(&name, &columns),
        Statement::Insert { table, values } => insert_row(&table, values),
        Statement::Select {
            columns,
            table,
            condition,
        } => select_rows(&table, &columns, condition.as_ref()),
        Statement::Update {
            table,
            set_col,
            set_val,
            condition,
        } => update_rows(&table, &set_col, set_val, &condition),
        Statement::Delete { table, condition } => delete_rows(&table, &condition),
    }
}
