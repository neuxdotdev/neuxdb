use crate::error::Result;
use crate::lib::service::insert_row;
use crate::types::Value;
pub fn execute(table: String, values: Vec<String>) -> Result<()> {
    let vals = values
        .into_iter()
        .map(|v| Value::from(v.as_str()))
        .collect();
    insert_row(&table, vals)
}
