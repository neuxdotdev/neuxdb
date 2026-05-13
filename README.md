# NeuxDB

**Embedded encrypted database for Rust – simple, secure, and portable.**

[![Crates.io](https://img.shields.io/crates/v/neuxdb)](https://crates.io/crates/neuxdb)
[![Docs](https://img.shields.io/docsrs/neuxdb)](https://docs.rs/neuxdb)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 1.85+](https://img.shields.io/badge/rustc-1.85+-orange.svg)](https://www.rust-lang.org)

NeuxDB is a **single‑file, encrypted, embedded database** inspired by SQLite’s simplicity but with a
strong focus on security and a minimal Rust API. Instead of SQL, you work directly with **tables,
columns, and rows** using plain Rust functions – no query language required.

Data is stored in **Pipe‑Separated Values (PSV)** format and protected with
**[age encryption](https://age-encryption.org)**, **HMAC‑SHA256 integrity checks**, and
**exclusive file locking**. Ideal for CLI tools, desktop apps, IoT devices, and any scenario
where you need a local database that *just works*.

---

## Key Features

-  **Full encryption at rest** – age‑encrypted file with passphrase‑derived keys (PBKDF2).
-  **Integrity guaranteed** – HMAC‑SHA256 header prevents undetected tampering.
-  **Concurrency safe** – Exclusive file locking (`fs2`) ensures no two processes corrupt the file.
-  **Strongly typed columns** – `Int`, `Text`, `Bool`, `Float` with strict validation.
-  **Multi‑format export** – CSV, PSV, JSON, HTML, Markdown, SQLite dump (plain or encrypted).
-  **Import support** – CSV, PSV, and JSON can be imported directly into a table.
-  **Transaction log** – Every insert/update/delete is recorded, with automatic archiving.
-  **Interactive HTML export** – Generated HTML tables include live search, filter, and sorting (no JavaScript frameworks needed).
-  **Zero boilerplate** – Create a database, add tables, and start inserting rows – all via safe, obvious methods.
-  **Single file** – Everything lives in one `.ndbx` file; easy to backup, copy, or share.

---

## Installation

```toml
[dependencies]
neuxdb = "0.3.1"
```

NeuxDB uses a few well‑known cryptographic libraries (`age-crypto`, `sha2`, `hmac`, `pbkdf2`) and
standard Rust crates for serialization (`serde`, `serde_json`) and file operations (`fs2`, `tempfile`).
All are automatically pulled in.

---

## Quick Start

```rust
use neuxdb::*;

fn main() -> neuxdb::Result<()> {
    // Create (or open) a password‑protected database
    let mut db = Database::create("my_app.ndbx", "strongpassword")?;

    // Define a table with typed columns
    db.create_table("users", vec![
        ("id",    ColumnType::Int),
        ("name",  ColumnType::Text),
        ("age",   ColumnType::Int),
    ])?;

    // Insert rows
    db.insert("users", vec![1.into(), "Alice".into(), 30.into()])?;
    db.insert("users", vec![2.into(), "Bob".into(),  25.into()])?;

    // Query all rows
    let rows = db.select("users", None, None)?;
    for row in &rows {
        println!("{:?}", row); // [Int(1), Text("Alice"), Int(30)]
    }

    // Update with a filter
    db.update("users", &|row| row[0] == Value::Int(1), "age", Value::Int(31))?;

    // Delete
    db.delete("users", &|row| row[0] == Value::Int(2))?;

    // Export table to JSON (or CSV, HTML, etc.)
    let json = db.export_table("users", ExportFormat::Json, false, None)?;
    std::fs::write("users.json", &json)?;

    // Save changes to disk
    db.commit()?;

    Ok(())
}
```

For a complete, runnable example including import from CSV and interactive HTML export,
see [`examples/demo.rs`](examples/demo.rs).  
Run it with:

```bash
cargo run --example demo
```

---

## API Overview

All functionality is exposed through the [`Database`] struct.

| Category | Method | Description |
|----------|--------|-------------|
| **Lifecycle** | `create(path, passphrase)` | Create a new `.ndbx` file |
| | `open(path, passphrase)` | Open an existing file |
| | `commit()` | Write in‑memory changes to disk |
| **Tables** | `create_table(name, columns)` | Add a table with typed columns |
| | `drop_table(name)` | Remove a table |
| | `list_tables()` | List all table names |
| | `table_schema(name)` | Get column names and types |
| **Data CRUD** | `insert(table, row)` | Add a row |
| | `select(table, columns, filter)` | Query rows (optional projection & filter) |
| | `update(table, filter, set_col, new_val)` | Modify matching rows |
| | `delete(table, filter)` | Remove matching rows |
| **Export/Import** | `export_table(table, format, encrypt, pass)` | Export table to CSV/PSV/JSON/HTML/MD/SQL (optionally encrypted) |
| | `import_table(name, data, format, encrypted, pass)` | Import from CSV/PSV/JSON (optionally encrypted) |
| **Meta** | `logs()` | Read transaction log |
| | `export_json()` | Dump entire database state as JSON |

Full API documentation is available on [docs.rs](https://docs.rs/neuxdb).

---

## Security & Integrity

NeuxDB is designed to keep your data safe even when the file is stored in untrusted locations.

- **Encryption**: The whole database is encrypted with age’s scrypt‑based passphrase encryption.
- **Key derivation**: Passphrase is strengthened with PBKDF2‑HMAC‑SHA256 (100,000 iterations).
- **Integrity**: Every write embeds an HMAC‑SHA256 of the plaintext, verified on every read.
- **Memory safety**: Passphrase and decrypted data are zeroized after use (`zeroize` crate).
- **File locking**: Exclusive advisory lock (`flock`) prevents concurrent corruption.
- **Anti‑tamper**: Any modification of the encrypted file will be detected and rejected.

---

## Supported Formats

### Export

| Format | `ExportFormat` | Notes |
|--------|----------------|-------|
| CSV | `Csv` | Comma‑separated |
| PSV | `Psv` | Pipe‑separated (native) |
| JSON | `Json` | Array of objects |
| HTML | `Html` | Interactive table with search & sort |
| Markdown | `Markdown` | GitHub‑flavoured table |
| SQLite Dump | `SqliteDump` | `CREATE TABLE` + `INSERT` statements |

Exports can be **encrypted** with the same passphrase (or a different one).

### Import

| Format | `ExportFormat` | Notes |
|--------|----------------|-------|
| CSV | `Csv` | First row is header |
| PSV | `Psv` | Pipe‑separated, first row is header |
| JSON | `Json` | Array of objects, keys become column names |

Imported data can be **encrypted** – NeuxDB will decrypt and validate it before ingestion.

---

## Architecture

```
src/
├── lib.rs           # Crate root, re‑exports
├── database.rs      # Core Database struct, locking, commit
├── table.rs         # TableSchema, TableData, ColumnDef, ColumnType
├── data.rs          # Low‑level CRUD operations
├── export.rs        # Export to multiple formats
├── import.rs        # Import from CSV, PSV, JSON
├── crypto.rs        # Age encryption + HMAC integrity
├── log.rs           # Transaction log
├── config.rs        # Constants and versioning
├── error.rs         # Unified error type
└── types.rs         # Value, ExportFormat, ColumnType
```

NeuxDB does **not** include a SQL parser – you interact with the database through safe, typed Rust
functions. This keeps the library small, fast, and easy to audit.

---

## Why NeuxDB?

| Feature | SQLite | NeuxDB |
|---------|--------|--------|
| Encryption | Requires extension (SQLCipher) or external tool | **Built‑in age encryption** |
| Integrity | Basic checksums | **HMAC‑SHA256 per write** |
| API | SQL strings | **Native Rust functions** |
| Type system | Flexible (manifest typing) | **Strict column types** |
| Export formats | CSV, JSON, … (via extensions) | **CSV, PSV, JSON, HTML, MD, SQL dump** |
| Dependencies | C library required | **Pure Rust** |

NeuxDB is *not* a replacement for SQLite in high‑concurrency or massive‑scale scenarios.  
It is the perfect choice when you need a **simple, secure, local database** that you can read,
write, and export without worrying about server setup or complex configuration.

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).  
For major changes, open an issue first to discuss what you would like to change.

---

## License

NeuxDB is licensed under the [MIT License](LICENSE).  
Dependencies are used under their respective licenses.