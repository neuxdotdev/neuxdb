use crate::config;
use crate::error::{NeuxDbError, Result};
use crate::types::TableSchema;
use std::fs;
pub(super) fn save_schema(name: &str, schema: &TableSchema) -> Result<()> {
    let schema_path = config::schema_path(name)?;
    let content =
        serde_json::to_string_pretty(schema).map_err(|e| NeuxDbError::Schema(e.to_string()))?;
    fs::write(schema_path, content)?;
    Ok(())
}
