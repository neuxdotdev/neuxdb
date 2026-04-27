use crate::config;
use crate::error::Result;
pub fn drop_table(name: &str) -> Result<()> {
    let table_path = config::table_path(name)?;
    let schema_path = config::schema_path(name)?;
    if table_path.exists() {
        std::fs::remove_file(&table_path)?;
    }
    if schema_path.exists() {
        std::fs::remove_file(&schema_path)?;
    }
    Ok(())
}
