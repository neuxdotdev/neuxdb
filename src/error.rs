use thiserror::Error;
#[derive(Debug, Error)]
pub enum DbError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV Error: {0}")]
    Csv(#[from] csv::Error),
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Table '{0}' not found")]
    TableNotFound(String),
    #[error("Table '{0}' already exists")]
    TableExists(String),
    #[error("Column '{0}' not found")]
    ColumnNotFound(String),
    #[error("Parse Error: {0}")]
    Parse(String),
    #[error("Type Mismatch: {0}")]
    TypeMismatch(String),
    #[error("Lock Error: {0}")]
    Lock(String),
    #[error("Invalid Input: {0}")]
    InvalidInput(String),
}
pub type Result<T> = std::result::Result<T, DbError>;
