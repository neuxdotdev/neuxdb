use crate::error::Result;
use crate::lib::service::{parse_condition, select_rows};
pub fn execute(table: String, columns: Vec<String>, r#where: Option<String>) -> Result<()> {
    let condition = if let Some(cond_str) = r#where {
        parse_condition(&cond_str)?
    } else {
        None
    };
    select_rows(&table, &columns, condition.as_ref())
}
