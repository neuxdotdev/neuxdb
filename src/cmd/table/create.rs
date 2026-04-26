use crate::error::Result;
use crate::lib::service::create_table;
pub fn execute(table: String, columns: Vec<String>) -> Result<()> {
    create_table(&table, &columns)
}
