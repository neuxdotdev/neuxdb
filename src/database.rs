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
