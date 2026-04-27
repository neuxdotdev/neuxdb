use crate::error::{NeuxDbError, Result};
use std::path::Path;
pub fn sanitize_table_name(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err(NeuxDbError::Parse("Table name cannot be empty".into()));
    }
    let path = Path::new(name);
    if path.components().any(|c| {
        matches!(
            c,
            std::path::Component::ParentDir | std::path::Component::RootDir
        )
    }) {
        return Err(NeuxDbError::Parse(format!(
            "Path traversal not allowed: '{}'",
            name
        )));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(NeuxDbError::Parse(format!(
            "Invalid character in table name '{}'",
            name
        )));
    }
    Ok(name.to_string())
}
