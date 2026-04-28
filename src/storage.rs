use crate::config;
use crate::error::{DbError, Result};
use crate::types::{ColumnType, Row, Schema, Value};
use csv::{ReaderBuilder, WriterBuilder};
use fs2::FileExt;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Seek, SeekFrom};
fn table_path_local(name: &str) -> Result<std::path::PathBuf> {
    sanitize(name)?;
    Ok(config::table_path(name))
}
fn schema_path_local(name: &str) -> Result<std::path::PathBuf> {
    sanitize(name)?;
    Ok(config::schema_path(name))
}
fn sanitize(name: &str) -> Result<()> {
    if name.is_empty() || name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        return Err(DbError::InvalidInput("Invalid table name".into()));
    }
    Ok(())
}
pub fn transact<F>(name: &str, f: F) -> Result<()>
where
    F: FnOnce(&Schema, &mut Vec<Row>) -> Result<()>,
{
    let tp = table_path_local(name)?;
    let sp = schema_path_local(name)?;
    if !tp.exists() {
        return Err(DbError::TableNotFound(name.into()));
    }
    let schema: Schema = serde_json::from_str(&fs::read_to_string(&sp)?)?;
    let file = OpenOptions::new().read(true).write(true).open(&tp)?;
    file.lock_exclusive()
        .map_err(|e| DbError::Lock(e.to_string()))?;
    let mut rows = read_csv(&file, &schema)?;
    f(&schema, &mut rows)?;
    write_csv(&file, &schema.columns, &rows)?;
    Ok(())
}
pub fn read_only(name: &str) -> Result<(Schema, Vec<String>, Vec<Row>)> {
    let tp = table_path_local(name)?;
    let sp = schema_path_local(name)?;
    if !tp.exists() {
        return Err(DbError::TableNotFound(name.into()));
    }
    let file = File::open(&tp)?;
    file.lock_shared()
        .map_err(|e| DbError::Lock(e.to_string()))?;
    let schema: Schema = serde_json::from_str(&fs::read_to_string(&sp)?)?;
    let headers = schema.columns.clone();
    let rows = read_csv(&file, &schema)?;
    Ok((schema, headers, rows))
}
pub fn create_table(name: &str, columns: Vec<String>) -> Result<()> {
    let tp = table_path_local(name)?;
    let sp = schema_path_local(name)?;
    if tp.exists() {
        return Err(DbError::TableExists(name.into()));
    }
    let schema = Schema::new(columns);
    let json = serde_json::to_string_pretty(&schema)?;
    fs::write(&sp, json)?;
    let file = File::create(&tp)?;
    file.lock_exclusive()
        .map_err(|e| DbError::Lock(e.to_string()))?;
    let mut wtr = WriterBuilder::new()
        .delimiter(config::get_delimiter())
        .from_writer(BufWriter::new(&file));
    wtr.write_record(&schema.columns)?;
    wtr.flush()?;
    Ok(())
}
pub fn drop_table(name: &str) -> Result<()> {
    let tp = table_path_local(name)?;
    let sp = schema_path_local(name)?;
    if tp.exists() {
        fs::remove_file(tp)?;
    }
    if sp.exists() {
        fs::remove_file(sp)?;
    }
    Ok(())
}
pub fn list_tables() -> Result<Vec<String>> {
    let mut tables = Vec::new();
    let ext = config::get_table_ext();
    for entry in fs::read_dir(config::get_base_path())? {
        let path = entry?.path();
        if path
            .extension()
            .map(|e| e.to_string_lossy() == ext)
            .unwrap_or(false)
        {
            if let Some(stem) = path.file_stem() {
                tables.push(stem.to_string_lossy().to_string());
            }
        }
    }
    tables.sort();
    Ok(tables)
}
fn read_csv(file: &File, schema: &Schema) -> Result<Vec<Row>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(config::get_delimiter())
        .has_headers(true)
        .from_reader(BufReader::new(file));
    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut row = Vec::new();
        for (i, field) in record.iter().enumerate() {
            let val = match schema.types.get(i) {
                Some(ColumnType::Int) => field
                    .parse::<i64>()
                    .map(Value::Int)
                    .unwrap_or_else(|_| Value::Text(field.to_string())),
                _ => Value::Text(field.to_string()),
            };
            row.push(val);
        }
        rows.push(row);
    }
    Ok(rows)
}
fn write_csv(file: &File, headers: &[String], rows: &[Row]) -> Result<()> {
    let mut file = file.try_clone()?;
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    let mut wtr = WriterBuilder::new()
        .delimiter(config::get_delimiter())
        .from_writer(BufWriter::new(&file));
    wtr.write_record(headers)?;
    for row in rows {
        let rec: Vec<String> = row.iter().map(|v| v.to_string()).collect();
        wtr.write_record(&rec)?;
    }
    wtr.flush()?;
    Ok(())
}
