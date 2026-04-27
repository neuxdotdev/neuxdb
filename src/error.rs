use thiserror::Error;
#[derive(Debug, Error)]
pub enum NeuxDbError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Table '{0}' not found")]
    TableNotFound(String),
    #[error("Table '{0}' already exists")]
    TableAlreadyExists(String),
    #[error("Column '{0}' not found in table '{1}'")]
    ColumnNotFound(String, String),
    #[error("Value count mismatch: expected {expected}, got {actual}")]
    ValueCountMismatch { expected: usize, actual: usize },
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Schema error: {0}")]
    Schema(String),
    #[error("Lock error: {0}")]
    Lock(String),
    #[error("Type mismatch: expected {expected} for column '{column}', got {found}")]
    TypeMismatch {
        expected: crate::types::ColumnType,
        column: String,
        found: crate::types::Value,
    },
    #[error("Duplicate column '{0}' in CREATE TABLE")]
    DuplicateColumn(String),
    #[error("Invalid LIKE pattern: {0}")]
    InvalidLikePattern(String),
}
pub type Result<T> = std::result::Result<T, NeuxDbError>;
