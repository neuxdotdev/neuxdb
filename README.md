# NeuxDb

**Version:** 0.1.0

NeuxDb is a super simple, fast, and tight embedded database library. NeuxDb stores data in CSV format with a pipe delimiter (`|`) and schemas in JSON format.

Designed with the "Single Function Interface" philosophy, NeuxDb eliminates boilerplate complexity by providing a single primary function: `run(sql)`.

---

## Key Features

- **Super Simple API**: Simply call `neuxdb::run("SQL")`.
- **Type-Safe**: Supports `Int` and `Text` data types with automatic validation.
- **Concurrency Safe**: Uses exclusive file locking (`flock`) to prevent race conditions.
- **SQL-Like**: Supports `CREATE`, `INSERT`, `SELECT`, `UPDATE`, `DELETE`, `DROP`, and `SHOW TABLES`.
- **Zero Dependency Config**: No complex configuration required, simply specify the data folder.

---

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
neuxdb = "0.1.0"
```
````

_(Ensure dependencies `thiserror`, `serde`, `serde_json`, `csv`, and `fs2` are present)_

---

## Quick Start

```rust
use neuxdb::{self, run, init, Result};

fn main() -> Result<()> {
// 1. Initialize data folder (default: ./data)
init()?;

// 2. Run the SQL command directly
run("CREATE TABLE users (id, name)")?;
run("INSERT INTO users VALUES (1, 'Alice')")?;
run("INSERT INTO users VALUES (2, 'Bob')")?;

// 3. Query data
let output = run("SELECT * FROM users")?;
println!("{}", output);

// 4. Update with conditions
run("UPDATE users SET name = 'Alicia' WHERE id = 1")?;

Ok(())
}
```

---

## API Reference

### Main Functions

#### `init() -> Result<()>`

Creates a data folder if it doesn't already exist. The default folder is `data/`, which can be changed via the `NEUXDB_DATA_DIR` environment variable.

#### `run(sql: &str) -> Result<String>`

The main function for executing SQL commands.

- **Input**: SQL command string.
- **Output**:
- `Ok(String)`: The result of the operation. For `SELECT`, it's a formatted table. For other operations, it's a success message.
- `Err(NeuxDbError)`: If an error occurs (e.g., table not found, incorrect syntax).

---

## SQL Syntax Reference

NeuxDb supports a clean and consistent subset of SQL.

### 1. Data Definition

```sql
CREATE TABLE table_name (column1, column2, ...);
DROP TABLE table_name;
SHOW TABLES;
```

### 2. Data Manipulation

**Insert**

```sql
INSERT INTO table_name VALUES (value1, value2);
-- Example of mixed types:
INSERT INTO users VALUES (1, 'Alice');
```

**Select**

```sql
SELECT * FROM table_name;
SELECT column1 FROM table_name WHERE id = 1;
```

_Support:_

- Columns: `*` or a specific list.
- Operators: `=`, `!=` (or `<>`), `<`, `>`, `<=`, `>=`, `LIKE`.
- Logic: `AND`, `OR`.
- Wildcards (LIKE): `%` (any character), `_` (single character).

**Update**

```sql
UPDATE table_name SET column = new_value WHERE condition;
```

_Note: `WHERE` is required for security (prevents accidental mass updates)._

**Delete**

```sql
DELETE FROM table_name WHERE condition;
```

_Note: `WHERE` is required._

---

## Data Types

NeuxDb automatically infers data types:

- **Int**: Whole number (e.g., `1`, `500`). Stored as `i64`.
- **Text**: String enclosed in quotes (e.g., `'Alice'`).

**Loose Typing:**
NeuxDb uses loose comparison. If you compare a Text column with a number (Int), the comparison will be based on their string representations.
_Example:_ `WHERE id = 1` will match the text column `"1"`.

---

## Error Handling

All errors are returned in the `neuxdb::DbError` enum.

```rust
use neuxdb::{run, DbError};

match run("SELECT * FROM ghost") {
Ok(msg) => println!("{}", msg),
Err(e) => {
match e {
DbError::TableNotFound(name) => eprintln!("Table {} does not exist!", name),
DbError::Parse(msg) => eprintln!("SQL Error: {}", msg),
_ => eprintln!("Other error: {}", e),
}
}
}
```

---

## Configuration

Using environment variables:

- `NEUXDB_DATA_DIR`: Absolute or relative path to the data storage folder. Default: `"data"`.

---

## Example `SELECT` Output

The `run` function returns a neat table string:

```
id | name
---+------
1 | Alice
2 | Bob
```

If no results:

```
id | name
---+------
(0 rows)
```

---

## Internal Architecture (Overview)

This library is structured in a flat and modular way behind the scenes:

1. **Parser**: Converts SQL strings to AST (`Statement`).
2. **Executor**: Executes business logic and calls storage.
3. **Storage**: Handles file I/O, CSV, and locking to ensure data integrity.

All of this complexity is abstracted into a single `run()` function.
