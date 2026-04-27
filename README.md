# NeuxDb API Documentation

Version: 0.1.0

NeuxDb is an embedded database library that stores data in pipe-delimited CSV files with per-table JSON schemas. It provides a simple SQL-like query language for table creation, data manipulation, and retrieval.

---

## Table of Contents

- [Re-exports](#re-exports)
- [Configuration `neuxdb::config`](#configuration-neuxdbconfig)
    - [`delimiter_byte`](#fn-delimiter_byte)
    - [`delimiter_char`](#fn-delimiter_char)
    - [`ensure_data_dir`](#fn-ensure_data_dir)
    - [`sanitize_table_name`](#fn-sanitize_table_name)
    - [`table_path`](#fn-table_path)
    - [`schema_path`](#fn-schema_path)
- [Types `neuxdb::types`](#types-neuxdbypes)
    - [`Value`](#enum-value)
    - [`Row`](#type-row)
    - [`ColumnType`](#enum-columntype)
    - [`TableSchema`](#struct-tableschema)
- [Error Handling `neuxdb::error`](#error-handling-neuxdberror)
    - [`NeuxDbError`](#enum-neuxdberror)
    - [`Result`](#type-result)
- [Core Compiler `neuxdb::core::compiler`](#core-compiler-neuxdbcorecompiler)
    - [`parse`](#fn-parse)
- [Core Syntax `neuxdb::core::syntax`](#core-syntax-neuxdbcoresyntax)
    - [`Statement`](#enum-statement)
    - [`WhereClause`](#enum-whereclause)
    - [`ComparisonOp`](#enum-comparisonop)
    - [`split_quoted`](#fn-split_quoted)
- [Core Service `neuxdb::core::service`](#core-service-neuxdbcoreservice)
    - [`create_table`](#fn-create_table)
    - [`drop_table`](#fn-drop_table)
    - [`show_tables`](#fn-show_tables)
    - [`insert_row`](#fn-insert_row)
    - [`select_rows`](#fn-select_rows)
    - [`update_rows`](#fn-update_rows)
    - [`delete_rows`](#fn-delete_rows)
    - [`format_table`](#fn-format_table)
    - [`parse_condition`](#fn-parse_condition)
    - [`parse_assignment`](#fn-parse_assignment)
    - [`run_script`](#fn-run_script)
- [Core Storage `neuxdb::core::storage`](#core-storage-neuxdbcorestorage)
    - [`read_table`](#fn-read_table)
    - [`write_table`](#fn-write_table)
    - [`create_table_schema`](#fn-create_table_schema)
- [Examples](#examples)

---

## Re-exports

All essential items are re-exported at the crate root:

```rust
pub use config::*;
pub use core::compiler::parse;
pub use core::service::*;
pub use core::storage::{read_table, write_table};
pub use core::syntax::{ComparisonOp, Statement, WhereClause};
pub use error::{NeuxDbError, Result};
pub use types::{ColumnType, Row, TableSchema, Value};
```

---

## Configuration `neuxdb::config`

### fn delimiter_byte

```rust
pub fn delimiter_byte() -> u8
```

Returns the byte value of the field delimiter used in data files (currently `|`, as `u8`).

---

### fn delimiter_char

```rust
pub fn delimiter_char() -> char
```

Returns the character value of the field delimiter used in data files (currently `'|'`).

---

### fn ensure_data_dir

```rust
pub fn ensure_data_dir() -> Result<()>
```

Ensures that the configured data directory exists. If the directory does not exist, it will be created automatically. The data directory location can be controlled by the environment variable `NEUXDB_DATA_DIR`; if not set, `"data"` is used.

**Errors:**  
Returns `NeuxDbError::Io` if the directory cannot be created.

---

### fn sanitize_table_name

```rust
pub fn sanitize_table_name(name: &str) -> Result<String>
```

Validates and sanitizes a table name.

- Rejects empty strings.
- Rejects path traversal sequences (e.g., `..`, absolute paths).
- Only allows ASCII alphanumeric characters, underscores, and hyphens.

**Errors:**  
Returns `NeuxDbError::Parse` if the name is invalid.

---

### fn table_path

```rust
pub fn table_path(name: &str) -> Result<PathBuf>
```

Constructs the path to the CSV data file for a given table. The file extension is `.nxdb`.

**Errors:**  
Propagates errors from `sanitize_table_name`.

---

### fn schema_path

```rust
pub fn schema_path(name: &str) -> Result<PathBuf>
```

Constructs the path to the JSON schema file for a given table. The schema file has extension `.schema.json`.

**Errors:**  
Propagates errors from `sanitize_table_name`.

---

## Types `neuxdb::types`

### enum Value

```rust
pub enum Value {
    Text(String),
    Int(i64),
}
```

Represents a single cell value. Two variants:

- `Text(String)` – string data.
- `Int(i64)` – signed 64-bit integer.

Implements:

- `From<&str>` and `From<String>` – converts from string; if parsing to integer succeeds, creates `Int`, otherwise `Text`.
- `Display` – formats the value as a string.

Methods:

- `as_text(&self) -> Option<&str>` – extracts the string if `Text`.
- `as_int(&self) -> Option<i64>` – extracts the integer if `Int`.
- `to_like_string(&self) -> String` – returns a string representation for LIKE pattern matching.

---

### type Row

```rust
pub type Row = Vec<Value>;
```

A row of data is a vector of `Value`s.

---

### enum ColumnType

```rust
pub enum ColumnType {
    Text,
    Int,
}
```

Defines the expected type of a column.

Implements `Display`, `Serialize`, `Deserialize`.

---

### struct TableSchema

```rust
pub struct TableSchema {
    pub columns: Vec<String>,
    pub types:  Vec<ColumnType>,
}
```

Describes the structure of a table: column names and their types.

Implements `Serialize`, `Deserialize`.

Methods:

- `new(columns: Vec<String>) -> Self` – creates a schema where all columns default to `ColumnType::Text`.
- `validate_value(&self, col_index: usize, value: &Value) -> Result<()>` – checks that the value matches the column’s type. Returns `NeuxDbError::TypeMismatch` if not.

---

## Error Handling `neuxdb::error`

### enum NeuxDbError

```rust
pub enum NeuxDbError {
    Io(#[from] std::io::Error),
    Csv(#[from] csv::Error),
    TableNotFound(String),
    TableAlreadyExists(String),
    ColumnNotFound(String, String),
    ValueCountMismatch { expected: usize, actual: usize },
    Parse(String),
    Schema(String),
    Lock(String),
    TypeMismatch { expected: ColumnType, column: String, found: Value },
    DuplicateColumn(String),
    InvalidLikePattern(String),
}
```

All errors produced by the library are variants of this enum. Each variant carries contextual information.

### type Result

```rust
pub type Result<T> = std::result::Result<T, NeuxDbError>;
```

Type alias for results returned by NeuxDb functions.

---

## Core Compiler `neuxdb::core::compiler`

### fn parse

```rust
pub fn parse(sql: &str) -> Result<Statement>
```

Parses a SQL-like statement string into a `Statement` enum. The parser is case‑insensitive for keywords and supports:

- `CREATE TABLE name (col1, col2, ...)`
- `DROP TABLE name`
- `SHOW TABLES`
- `INSERT INTO name VALUES (val1|val2|...)`
- `SELECT columns FROM name [WHERE condition]`
- `UPDATE name SET col=val WHERE condition`
- `DELETE FROM name WHERE condition`

Conditions (`WHERE`) support `=`, `!=`, `<>`, `<`, `>`, `<=`, `>=`, `LIKE`, and combinations with `AND`, `OR`.

**Errors:** Returns `NeuxDbError::Parse` with a detailed message if the input is invalid.

---

## Core Syntax `neuxdb::core::syntax`

### enum Statement

```rust
pub enum Statement {
    CreateTable { name: String, columns: Vec<String> },
    DropTable   { name: String },
    ShowTables,
    Insert      { table: String, values: Vec<Value> },
    Select      { columns: Vec<String>, table: String, condition: Option<WhereClause> },
    Update      { table: String, set_col: String, set_val: Value, condition: WhereClause },
    Delete      { table: String, condition: WhereClause },
}
```

Represents a parsed statement. Variants correspond to the supported commands.

### enum WhereClause

```rust
pub enum WhereClause {
    Condition {
        column:    String,
        operator:  ComparisonOp,
        value:     Value,
    },
    And(Box<WhereClause>, Box<WhereClause>),
    Or(Box<WhereClause>, Box<WhereClause>),
}
```

Expresses a filter condition. Supports nesting with `And`/`Or`.

### enum ComparisonOp

```rust
pub enum ComparisonOp {
    Eq,   // =
    Ne,   // != or <>
    Lt,   // <
    Gt,   // >
    Le,   // <=
    Ge,   // >=
    Like, // LIKE
}
```

Operators available in `WhereClause`.

### fn split_quoted

```rust
pub fn split_quoted(s: &str, delim: char) -> Vec<String>
```

Splits a string by a delimiter, respecting single-quoted substrings. Used internally for parsing value lists.

---

## Core Service `neuxdb::core::service`

### fn create_table

```rust
pub fn create_table(name: &str, columns: &[String]) -> Result<()>
```

Creates a new table with the given columns. All columns are created with type `Text`. A corresponding schema file is saved, and an empty data file is written with the column headers.

**Errors:**

- `TableAlreadyExists` if the table already exists.
- I/O errors during file creation.

---

### fn drop_table

```rust
pub fn drop_table(name: &str) -> Result<()>
```

Deletes the table’s data file (`name.nxdb`) and its schema file (`name.schema.json`).

**Errors:** I/O errors if removal fails.

---

### fn show_tables

```rust
pub fn show_tables() -> Result<Vec<String>>
```

Returns a sorted list of existing table names (without extensions).

**Errors:** I/O errors reading the data directory.

---

### fn insert_row

```rust
pub fn insert_row(table: &str, values: Vec<Value>) -> Result<()>
```

Inserts a single row into an existing table.

- Validates the number of values matches the columns.
- Validates each value against the column’s type (see `TableSchema::validate_value`).
- Writes the updated data back atomically.

**Errors:**

- `TableNotFound`
- `ValueCountMismatch`
- `TypeMismatch`
- I/O and lock errors.

---

### fn select_rows

```rust
pub fn select_rows(
    table: &str,
    columns: &[String],
    condition: Option<&WhereClause>
) -> Result<Vec<Row>>
```

Retrieves rows from a table.

- If columns contains `"*"`, all columns are returned.
- If a condition is provided, only matching rows are included.
- Conditions support all comparison operators (`=`, `!=`, `<`, etc.) and `LIKE` (with `%` and `_` wildcards), as well as `AND`/`OR` combinations.

**Errors:** `TableNotFound`, `ColumnNotFound`, I/O and lock errors.

---

### fn update_rows

```rust
pub fn update_rows(
    table: &str,
    set_col: &str,
    set_val: Value,
    condition: &WhereClause
) -> Result<usize>
```

Updates rows that match the condition by setting a column to a new value. Returns the number of modified rows.

**Errors:** `TableNotFound`, `ColumnNotFound`, I/O and lock errors.

---

### fn delete_rows

```rust
pub fn delete_rows(table: &str, condition: &WhereClause) -> Result<usize>
```

Deletes rows that match the condition. Returns the number of deleted rows.

**Errors:** `TableNotFound`, I/O and lock errors.

---

### fn format_table

```rust
pub fn format_table(columns: &[String], rows: &[Row]) -> String
```

Formats a header and data rows into a human-readable table with aligned columns and separators. If no rows are present, the output includes `(no rows)`.

Example output:

```
id | name
---+-----
1  | Alice
2  | Bob
```

---

### fn parse_condition

```rust
pub fn parse_condition(cond: &str) -> Result<Option<(String, Value)>>
```

Parses a simple `column=value` string (supporting single‑quoted values) into a tuple.  
Used for backward compatibility; prefer using `parse` and `WhereClause` directly.

---

### fn parse_assignment

```rust
pub fn parse_assignment(assign: &str) -> Result<(String, Value)>
```

Parses a `column=value` string into a tuple.

---

### fn run_script

```rust
pub fn run_script(path: &str, callback: impl Fn(&str) -> Result<()>) -> Result<()>
```

Reads a file line by line, skipping comments (`#`) and empty lines, and calls the callback for each SQL statement. If the callback returns an error, it is wrapped with a line number.

---

## Core Storage `neuxdb::core::storage`

### fn read_table

```rust
pub fn read_table(name: &str) -> Result<(Vec<String>, Vec<Row>)>
```

Reads the entire table, returning column headers and all rows.

- Acquires a shared lock while reading.
- Values are parsed according to the column types defined in the schema.

**Errors:** `TableNotFound`, I/O error, schema error, CSV error.

---

### fn write_table

```rust
pub fn write_table(name: &str, headers: &[String], rows: &[Row]) -> Result<()>
```

Writes headers and rows atomically to the table file.

- Creates a temporary file, writes the data, then renames it to the target path.
- Acquires an exclusive lock during write.

---

### fn create_table_schema

```rust
pub fn create_table_schema(name: &str, columns: &[String]) -> Result<()>
```

Creates a new table file with the given columns and writes the corresponding schema. Used internally by `create_table`.

---

## Examples

### Basic Table Creation and Query

```rust
use neuxdb::*;

// Ensure the data directory exists
ensure_data_dir().unwrap();

// Create a table
create_table("users", &["id".into(), "name".into()]).unwrap();

// Insert a row
insert_row("users", vec![Value::Text("1".into()), Value::Text("Alice".into())]).unwrap();

// Select all rows
let rows = select_rows("users", &["*".into()], None).unwrap();
let (headers, _) = read_table("users").unwrap();
println!("{}", format_table(&headers, &rows));
```

### Using WHERE Conditions

```rust
use neuxdb::*;

let condition = WhereClause::Condition {
    column:   "id".to_string(),
    operator: ComparisonOp::Eq,
    value:    Value::Text("1".into()),
};

let rows = select_rows("users", &["name".into()], Some(&condition)).unwrap();
for row in rows {
    println!("{}", row[0]);
}
```

### Drop and Show Tables

```rust
drop_table("users").unwrap();

let tables = show_tables().unwrap();
println!("Tables: {:?}", tables);
```
