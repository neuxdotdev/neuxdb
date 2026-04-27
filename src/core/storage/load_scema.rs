use super::save_chema::save_schema;
use crate::config;
use crate::error::{NeuxDbError, Result};
use crate::types::TableSchema;
use csv::ReaderBuilder;
use std::fs;
pub(crate) fn load_schema(name: &str) -> Result<TableSchema> {
    let schema_path = config::schema_path(name)?;
    if !schema_path.exists() {
        let path = config::table_path(name)?;
        if !path.exists() {
            return Err(NeuxDbError::TableNotFound(name.to_string()));
        }
        let data = fs::read_to_string(&path)?;
        let mut rdr = ReaderBuilder::new()
            .delimiter(config::delimiter_byte())
            .from_reader(data.as_bytes());
        let headers = rdr.headers()?.iter().map(|s| s.to_string()).collect();
        let schema = TableSchema::new(headers);
        save_schema(name, &schema)?;
        Ok(schema)
    } else {
        let content = fs::read_to_string(schema_path)?;
        let schema: TableSchema =
            serde_json::from_str(&content).map_err(|e| NeuxDbError::Schema(e.to_string()))?;
        Ok(schema)
    }
}
