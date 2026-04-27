use crate::core::storage::create_table_schema;
use crate::error::Result;
pub fn create_table(name: &str, columns: &[String]) -> Result<()> {
    create_table_schema(name, columns)
}
