use crate::error::Result;
use crate::lib::service::{parse_assignment, parse_condition, update_rows};
pub fn execute(table: String, set: String, r#where: String) -> Result<()> {
    let (set_col, set_val) = parse_assignment(&set)?;
    let (cond_col, cond_val) = parse_condition(&r#where)?
        .ok_or_else(|| crate::error::NeuxError::Parse("WHERE clause required".into()))?;
    update_rows(&table, &set_col, set_val, &(cond_col, cond_val))
}
