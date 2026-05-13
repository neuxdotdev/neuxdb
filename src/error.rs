use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Crypto error: {0}")]
    Crypto(String),
    #[error("Integrity error: {0}")]
    Integrity(String),
    #[error("Table '{0}' not found")]
    TableNotFound(String),
    #[error("Table '{0}' already exists")]
    TableExists(String),
    #[error("Column '{0}' not found")]
    ColumnNotFound(String),
    #[error("Invalid column index {0}")]
    InvalidColumnIndex(usize),
    #[error("Database not open")]
    NotOpen,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Invalid database format: {0}")]
    InvalidFormat(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Database is locked by another process")]
    DatabaseLocked,
    #[error("Version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u8, actual: u8 },
    #[error("Passphrase is too weak: {0}")]
    WeakPassphrase(String),
    #[error("Type mismatch: column '{column}' expects {expected} but got {actual}")]
    TypeMismatch {
        column: String,
        expected: String,
        actual: String,
    },
}
pub type Result<T> = std::result::Result<T, Error>;
