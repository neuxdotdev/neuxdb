```
.  # src
├── config.rs
├── crypto.rs
├── data.rs
├── database.rs
├── error.rs
├── lib.rs
├── log.rs
├── table.rs
├── types.rs
```
## src/lib.rs

```rust
pub mod config;
pub mod crypto;
pub mod data;
pub mod database;
pub mod error;
pub mod log;
pub mod table;
pub mod types;
pub use database::Database;
pub use error::{Error, Result};
pub use table::ColumnType;
pub use types::Value;
pub mod prelude {
    pub use super::{ColumnType, Database, Error, Result, Value};
}
```
## src/error.rs

```rust
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
```
## src/config.rs

```rust
pub const DELIMITER: u8 = b'|';
pub const HEADER_MAGIC: &str = "[NEUXDB:v1]";
pub const INTEGRITY_PREFIX: &str = "SHA=";
pub const FILE_EXTENSION: &str = "ndbx";
pub const MAX_TABLE_NAME_LEN: usize = 64;
pub const MAX_COLUMN_NAME_LEN: usize = 64;
pub const MAX_COLUMNS_PER_TABLE: usize = 100;
pub const MAX_LOG_ENTRIES: usize = 10_000;
pub const MIN_PASSPHRASE_LEN: usize = 8;
pub const ALLOWED_NAME_REGEX: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*$";
```
## src/crypto.rs

```rust
use crate::config;
use crate::error::{Error, Result};
use sha2::{Digest, Sha256};
pub fn sha256_hex(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}
pub fn seal(payload_json: &str) -> String {
    let hash = sha256_hex(payload_json);
    format!("{}SHA={}\n{}", config::HEADER_MAGIC, hash, payload_json)
}
pub fn unseal(payload: &str) -> Result<String> {
    let newline_pos = payload
        .find('\n')
        .ok_or_else(|| Error::InvalidFormat("Missing header newline".into()))?;
    let header = &payload[..newline_pos];
    let json = &payload[newline_pos + 1..];
    if !header.starts_with(config::HEADER_MAGIC) {
        return Err(Error::InvalidFormat("Invalid magic header".into()));
    }
    let expected_hash = header
        .trim_start_matches(config::HEADER_MAGIC)
        .trim_start_matches(config::INTEGRITY_PREFIX);
    let actual_hash = sha256_hex(json);
    if expected_hash != actual_hash {
        return Err(Error::Integrity("Data corrupted or tampered!".into()));
    }
    Ok(json.to_string())
}
pub fn encrypt(plaintext: &str, passphrase: &str) -> Result<Vec<u8>> {
    let sealed = seal(plaintext);
    let ciphertext = age_crypto::encrypt_with_passphrase(sealed.as_bytes(), passphrase)
        .map_err(|e| Error::Crypto(e.to_string()))?;
    Ok(ciphertext.to_vec())
}
pub fn decrypt(ciphertext: &[u8], passphrase: &str) -> Result<String> {
    let plain =
        age_crypto::decrypt_with_passphrase(ciphertext, passphrase).map_err(|e| match e {
            age_crypto::Error::Decrypt(_) => Error::InvalidPassword,
            _ => Error::Crypto(e.to_string()),
        })?;
    let sealed = String::from_utf8(plain)
        .map_err(|_| Error::InvalidFormat("Payload is not UTF-8".into()))?;
    unseal(&sealed)
}
```
## src/types.rs

```rust
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Int(i64),
    Text(String),
    Bool(bool),
    Float(f64),
    Null,
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Text(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Null => write!(f, "null"),
        }
    }
}
impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.to_string())
    }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}
impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}
```
## src/log.rs

```rust
use crate::config;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub action: String,
    pub table: String,
    pub details: Option<String>,
}
impl LogEntry {
    pub fn new(action: &str, table: &str, details: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            timestamp,
            action: action.to_string(),
            table: table.to_string(),
            details,
        }
    }
}
pub fn trim_logs_if_needed(logs: &mut Vec<LogEntry>) {
    if logs.len() > config::MAX_LOG_ENTRIES {
        let drain_up_to = logs.len() - config::MAX_LOG_ENTRIES;
        logs.drain(0..drain_up_to);
    }
}
```
## src/data.rs

```rust
use crate::error::{Error, Result};
use crate::log::LogEntry;
use crate::table::TableData;
use crate::types::Value;
pub fn insert(table: &mut TableData, row: Vec<Value>, logs: &mut Vec<LogEntry>) -> Result<()> {
    table.schema.validate_row(&row)?;
    table.rows.push(row.clone());
    logs.push(LogEntry::new(
        "INSERT",
        &table.schema.name,
        Some(format!("{:?}", row)),
    ));
    Ok(())
}
pub fn select(
    table: &TableData,
    columns: Option<Vec<&str>>,
    filter: Option<&dyn Fn(&[Value]) -> bool>,
) -> Result<Vec<Vec<Value>>> {
    let mut result = Vec::new();
    for row in &table.rows {
        if let Some(pred) = &filter {
            if !pred(row) {
                continue;
            }
        }
        if let Some(cols) = &columns {
            let indices: Vec<usize> = cols
                .iter()
                .map(|c| {
                    table
                        .schema
                        .columns
                        .iter()
                        .position(|col_def| col_def.name == *c)
                        .ok_or_else(|| Error::ColumnNotFound(c.to_string()))
                })
                .collect::<Result<_>>()?;
            let projected: Vec<Value> = indices.iter().map(|&i| row[i].clone()).collect();
            result.push(projected);
        } else {
            result.push(row.clone());
        }
    }
    Ok(result)
}
pub fn update(
    table: &mut TableData,
    filter: &dyn Fn(&[Value]) -> bool,
    set_col: &str,
    new_val: Value,
    logs: &mut Vec<LogEntry>,
) -> Result<usize> {
    let col_idx = table
        .schema
        .columns
        .iter()
        .position(|col_def| col_def.name == set_col)
        .ok_or_else(|| Error::ColumnNotFound(set_col.to_string()))?;
    let col_type = table.schema.columns[col_idx].col_type;
    if !col_type.validate(&new_val) {
        return Err(Error::TypeMismatch {
            column: set_col.to_string(),
            expected: col_type.name().to_string(),
            actual: match &new_val {
                Value::Int(_) => "INT",
                Value::Text(_) => "TEXT",
                Value::Bool(_) => "BOOL",
                Value::Float(_) => "FLOAT",
                Value::Null => "NULL",
            }
            .to_string(),
        });
    }
    let mut count = 0;
    for row in &mut table.rows {
        if filter(row) {
            row[col_idx] = new_val.clone();
            count += 1;
        }
    }
    if count > 0 {
        logs.push(LogEntry::new(
            "UPDATE",
            &table.schema.name,
            Some(format!("Set {} = {:?} on {} rows", set_col, new_val, count)),
        ));
    }
    Ok(count)
}
pub fn delete(
    table: &mut TableData,
    filter: &dyn Fn(&[Value]) -> bool,
    logs: &mut Vec<LogEntry>,
) -> Result<usize> {
    let original = table.rows.len();
    table.rows.retain(|r| !filter(r));
    let removed = original - table.rows.len();
    if removed > 0 {
        logs.push(LogEntry::new(
            "DELETE",
            &table.schema.name,
            Some(format!("Removed {} rows", removed)),
        ));
    }
    Ok(removed)
}
```
## src/database.rs

```rust
use crate::config;
use crate::crypto;
use crate::error::{Error, Result};
use crate::log::{self, LogEntry};
use crate::table::{ColumnDef, ColumnType, TableData, TableSchema};
use crate::types::Value;
use fs2::FileExt;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use zeroize::Zeroizing;
#[derive(Debug, Serialize, Deserialize)]
struct DbState {
    version: u8,
    tables: HashMap<String, TableData>,
    logs: Vec<LogEntry>,
}
impl DbState {
    fn new() -> Self {
        Self {
            version: 1,
            tables: HashMap::new(),
            logs: vec![],
        }
    }
    fn validate(&self) -> Result<()> {
        if self.version != 1 {
            return Err(Error::VersionMismatch {
                expected: 1,
                actual: self.version,
            });
        }
        for (name, table) in &self.tables {
            if name != &table.schema.name {
                return Err(Error::Integrity(format!(
                    "Table name mismatch: {} vs {}",
                    name, table.schema.name
                )));
            }
            for col in &table.schema.columns {
                if col.name.is_empty() || col.name.len() > config::MAX_COLUMN_NAME_LEN {
                    return Err(Error::InvalidInput(format!(
                        "Invalid column name: {}",
                        col.name
                    )));
                }
            }
        }
        Ok(())
    }
}
pub struct Database {
    path: PathBuf,
    passphrase: Zeroizing<String>,
    state: DbState,
    dirty: bool,
    lock_file: fs::File,
}
impl Database {
    fn validate_name(name: &str, max_len: usize, context: &str) -> Result<()> {
        let re = Regex::new(config::ALLOWED_NAME_REGEX).unwrap();
        if name.is_empty() || name.len() > max_len {
            return Err(Error::InvalidInput(format!(
                "{} name length must be 1..{}",
                context, max_len
            )));
        }
        if !re.is_match(name) {
            return Err(Error::InvalidInput(format!(
                "{} '{}' contains invalid characters. Allowed: letters, numbers, underscore, must start with letter or underscore",
                context, name
            )));
        }
        Ok(())
    }
    fn validate_path(path: &Path) -> Result<()> {
        if let Some(ext) = path.extension() {
            if ext != config::FILE_EXTENSION {
                return Err(Error::InvalidInput(format!(
                    "File extension must be .{}",
                    config::FILE_EXTENSION
                )));
            }
        } else {
            return Err(Error::InvalidInput(format!(
                "File must have .{} extension",
                config::FILE_EXTENSION
            )));
        }
        match fs::symlink_metadata(path) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    return Err(Error::InvalidInput("Symlinks are not allowed".into()));
                }
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    return Err(Error::Io(e));
                }
            }
        }
        Ok(())
    }
    fn atomic_write(path: &Path, data: &[u8]) -> Result<()> {
        let parent = path.parent().ok_or_else(|| {
            Error::Io(io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"))
        })?;
        fs::create_dir_all(parent)?;
        let mut temp = NamedTempFile::new_in(parent)?;
        temp.write_all(data)?;
        temp.flush()?;
        temp.persist(path).map_err(|e| Error::Io(e.error))?;
        Ok(())
    }
    pub fn create(path: impl Into<PathBuf>, passphrase: &str) -> Result<Self> {
        let path: PathBuf = path.into();
        Self::validate_path(&path)?;
        if path.exists() {
            return Err(Error::InvalidInput("Database file already exists".into()));
        }
        if passphrase.len() < config::MIN_PASSPHRASE_LEN {
            return Err(Error::WeakPassphrase(format!(
                "Minimum length is {}",
                config::MIN_PASSPHRASE_LEN
            )));
        }
        let state = DbState::new();
        let json = serde_json::to_string_pretty(&state)?;
        let encrypted = crypto::encrypt(&json, passphrase)?;
        Self::atomic_write(&path, &encrypted)?;
        let lock_file = fs::OpenOptions::new().read(true).write(true).open(&path)?;
        lock_file
            .try_lock_exclusive()
            .map_err(|_| Error::DatabaseLocked)?;
        Ok(Self {
            path,
            passphrase: Zeroizing::new(passphrase.to_string()),
            state,
            dirty: false,
            lock_file,
        })
    }
    pub fn open(path: impl Into<PathBuf>, passphrase: &str) -> Result<Self> {
        let path: PathBuf = path.into();
        Self::validate_path(&path)?;
        if !path.exists() {
            return Err(Error::Io(io::Error::new(
                io::ErrorKind::NotFound,
                "Database file not found",
            )));
        }
        let lock_file = fs::OpenOptions::new().read(true).write(true).open(&path)?;
        lock_file
            .try_lock_exclusive()
            .map_err(|_| Error::DatabaseLocked)?;
        let ciphertext = fs::read(&path)?;
        let json = crypto::decrypt(&ciphertext, passphrase)?;
        let state: DbState = serde_json::from_str(&json)?;
        state.validate()?;
        Ok(Self {
            path,
            passphrase: Zeroizing::new(passphrase.to_string()),
            state,
            dirty: false,
            lock_file,
        })
    }
    pub fn commit(&mut self) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }
        let json = serde_json::to_string_pretty(&self.state)?;
        let encrypted = crypto::encrypt(&json, &self.passphrase)?;
        Self::atomic_write(&self.path, &encrypted)?;
        self.dirty = false;
        Ok(())
    }
    pub fn create_table(&mut self, name: &str, columns: Vec<(&str, ColumnType)>) -> Result<()> {
        Self::validate_name(name, config::MAX_TABLE_NAME_LEN, "Table")?;
        if columns.is_empty() || columns.len() > config::MAX_COLUMNS_PER_TABLE {
            return Err(Error::InvalidInput(format!(
                "Table must have between 1 and {} columns",
                config::MAX_COLUMNS_PER_TABLE
            )));
        }
        let mut col_defs = Vec::new();
        let mut col_names_for_log = Vec::new();
        for (col_name, col_type) in &columns {
            Self::validate_name(col_name, config::MAX_COLUMN_NAME_LEN, "Column")?;
            col_defs.push(ColumnDef {
                name: col_name.to_string(),
                col_type: *col_type,
            });
            col_names_for_log.push((*col_name, *col_type));
        }
        if self.state.tables.contains_key(name) {
            return Err(Error::TableExists(name.to_string()));
        }
        let schema = TableSchema::new(name.to_string(), col_defs);
        let table = TableData::new(schema);
        self.state.tables.insert(name.to_string(), table);
        self.state.logs.push(LogEntry::new(
            "CREATE TABLE",
            name,
            Some(format!("Columns: {:?}", col_names_for_log)),
        ));
        log::trim_logs_if_needed(&mut self.state.logs);
        self.dirty = true;
        Ok(())
    }
    pub fn drop_table(&mut self, name: &str) -> Result<()> {
        if self.state.tables.remove(name).is_some() {
            self.state
                .logs
                .push(LogEntry::new("DROP TABLE", name, None));
            log::trim_logs_if_needed(&mut self.state.logs);
            self.dirty = true;
            Ok(())
        } else {
            Err(Error::TableNotFound(name.to_string()))
        }
    }
    pub fn list_tables(&self) -> Vec<String> {
        let mut names: Vec<String> = self.state.tables.keys().cloned().collect();
        names.sort();
        names
    }
    pub fn table_schema(&self, name: &str) -> Result<Vec<(String, ColumnType)>> {
        let table = self
            .state
            .tables
            .get(name)
            .ok_or_else(|| Error::TableNotFound(name.to_string()))?;
        Ok(table
            .schema
            .columns
            .iter()
            .map(|c| (c.name.clone(), c.col_type))
            .collect())
    }
    pub fn insert(&mut self, table: &str, row: Vec<Value>) -> Result<()> {
        let table_data = self
            .state
            .tables
            .get_mut(table)
            .ok_or_else(|| Error::TableNotFound(table.to_string()))?;
        crate::data::insert(table_data, row, &mut self.state.logs)?;
        log::trim_logs_if_needed(&mut self.state.logs);
        self.dirty = true;
        Ok(())
    }
    pub fn select(
        &self,
        table: &str,
        columns: Option<Vec<&str>>,
        filter: Option<&dyn Fn(&[Value]) -> bool>,
    ) -> Result<Vec<Vec<Value>>> {
        let table_data = self
            .state
            .tables
            .get(table)
            .ok_or_else(|| Error::TableNotFound(table.to_string()))?;
        crate::data::select(table_data, columns, filter)
    }
    pub fn update(
        &mut self,
        table: &str,
        filter: &dyn Fn(&[Value]) -> bool,
        set_col: &str,
        new_val: Value,
    ) -> Result<usize> {
        let table_data = self
            .state
            .tables
            .get_mut(table)
            .ok_or_else(|| Error::TableNotFound(table.to_string()))?;
        let count =
            crate::data::update(table_data, filter, set_col, new_val, &mut self.state.logs)?;
        if count > 0 {
            log::trim_logs_if_needed(&mut self.state.logs);
            self.dirty = true;
        }
        Ok(count)
    }
    pub fn delete(&mut self, table: &str, filter: &dyn Fn(&[Value]) -> bool) -> Result<usize> {
        let table_data = self
            .state
            .tables
            .get_mut(table)
            .ok_or_else(|| Error::TableNotFound(table.to_string()))?;
        let removed = crate::data::delete(table_data, filter, &mut self.state.logs)?;
        if removed > 0 {
            log::trim_logs_if_needed(&mut self.state.logs);
            self.dirty = true;
        }
        Ok(removed)
    }
    pub fn logs(&self) -> &[LogEntry] {
        &self.state.logs
    }
    pub fn export_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.state).map_err(Into::into)
    }
}
impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Database")
            .field("path", &self.path)
            .field("dirty", &self.dirty)
            .finish_non_exhaustive()
    }
}
impl Drop for Database {
    fn drop(&mut self) {
        if self.dirty {
            eprintln!(
                "Warning: Database '{}' has uncommitted changes!",
                self.path.display()
            );
        }
    }
}
```
## src/table.rs

```rust
use crate::error::{Error, Result};
use crate::types::Value;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    Int,
    Text,
    Bool,
    Float,
}
impl ColumnType {
    pub fn validate(&self, value: &Value) -> bool {
        match (self, value) {
            (ColumnType::Int, Value::Int(_)) => true,
            (ColumnType::Text, Value::Text(_)) => true,
            (ColumnType::Bool, Value::Bool(_)) => true,
            (ColumnType::Float, Value::Float(_)) => true,
            _ => false,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            ColumnType::Int => "INT",
            ColumnType::Text => "TEXT",
            ColumnType::Bool => "BOOL",
            ColumnType::Float => "FLOAT",
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub col_type: ColumnType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnDef>,
}
impl TableSchema {
    pub fn new(name: String, columns: Vec<ColumnDef>) -> Self {
        Self { name, columns }
    }
    pub fn validate_row(&self, row: &[Value]) -> Result<()> {
        if row.len() != self.columns.len() {
            return Err(Error::InvalidInput(format!(
                "Expected {} columns, got {}",
                self.columns.len(),
                row.len()
            )));
        }
        for (_idx, (col_def, val)) in self.columns.iter().zip(row.iter()).enumerate() {
            if !col_def.col_type.validate(val) {
                return Err(Error::TypeMismatch {
                    column: col_def.name.clone(),
                    expected: col_def.col_type.name().to_string(),
                    actual: match val {
                        Value::Int(_) => "INT",
                        Value::Text(_) => "TEXT",
                        Value::Bool(_) => "BOOL",
                        Value::Float(_) => "FLOAT",
                        Value::Null => "NULL",
                    }
                    .to_string(),
                });
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub schema: TableSchema,
    pub rows: Vec<Vec<Value>>,
}
impl TableData {
    pub fn new(schema: TableSchema) -> Self {
        Self {
            schema,
            rows: vec![],
        }
    }
}
```
