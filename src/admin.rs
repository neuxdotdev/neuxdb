use crate::config;
use crate::error::{DbError, Result};
use crate::storage;
use std::fs;
pub fn backup_table(table: &str) -> Result<String> {
    let src_path = config::table_path(table);
    let schema_path = config::schema_path(table);
    if !src_path.exists() {
        return Err(DbError::TableNotFound(table.into()));
    }
    let backup_path = src_path.with_extension("nxdb.bak");
    let backup_schema = schema_path.with_extension("json.bak");
    fs::copy(&src_path, &backup_path)?;
    fs::copy(&schema_path, &backup_schema)?;
    Ok(format!("Table '{}' backed up to {:?}", table, backup_path))
}
pub fn check_integrity(table: &str) -> Result<String> {
    match storage::read_only(table) {
        Ok((schema, headers, rows)) => {
            let mut issues = Vec::new();
            if schema.columns != headers {
                issues.push("Schema columns do not match CSV headers".to_string());
            }
            for (i, row) in rows.iter().enumerate() {
                if row.len() != headers.len() {
                    issues.push(format!(
                        "Row {} has {} columns, expected {}",
                        i,
                        row.len(),
                        headers.len()
                    ));
                }
            }
            if issues.is_empty() {
                Ok(format!("Table '{}' integrity OK.", table))
            } else {
                Ok(format!("Table '{}' has issues: {:?}", table, issues))
            }
        }
        Err(e) => Err(e),
    }
}
