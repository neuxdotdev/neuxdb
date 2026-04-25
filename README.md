# neuxdb

A super simple, embedded, encrypted database written in Rust. 
Like SQLite, but with pipe‑separated files and full encryption using the `age` ecosystem.

---

## Features

- File‑based – no server, no complex setup.
- Custom SQL‑like syntax (lowercase, minimalist).
- Pipe (`|`) as column delimiter – clean and easy to parse.
- Encrypted at rest (integration with `age-crypto` and `age-setup`).
- CRUD operations: `CREATE`, `INSERT`, `SELECT`, `UPDATE`, `DELETE`.
- Script execution from `.ndbx` files.
- Strong error handling, type‑safe `Value` (Text/Int).
- Built with pure Rust – fast, memory safe, and fearless concurrency.

---

## Quick Start

### 1. Build

```bash
git clone https://github.com/neuxdotdev/neuxdb.git
cd neuxdb
cargo build --release
cargo install --path .
```

The binary will be available at `./target/release/neuxdb`.

### 2. Basic Usage

```bash
# Create a table
neuxdb "create table users (id, name, email)"

# Insert a row (pipe separator)
neuxdb "insert into users values (1|'Alice'|alice@ex.com)"

# Select all
neuxdb "select * from users"

# Select with condition
neuxdb "select name, email from users where id=1"

# Update a row
neuxdb "update users set name='Budi' where id=1"

# Delete a row
neuxdb "delete from users where id=1"
```

### 3. Run a Script

Create a file `test.ndbx`:

```ndbx
create table users (id, name, email)
insert into users values (1|'Alice'|alice@ex.com)
select * from users
```

Then execute:

```bash
neuxdb run test.ndbx
```

---

## Syntax Reference

All commands are **case‑sensitive and lowercase**.

- **CREATE TABLE**  
  `create table name (col1, col2, ...)`

- **INSERT** (values separated by `|`)  
  `insert into name values (val1|val2|...)`

- **SELECT**  
  `select * from name`  
  `select col1, col2 from name where col=value`

- **UPDATE**  
  `update name set col=value where col=value`

- **DELETE**  
  `delete from name where col=value`

String values must be quoted with single quotes, e.g. `'hello'`.  
Numbers are automatically recognized as integers.

---

## Directory Structure

```
data/                    # all tables are stored here
  └── table_name.ndbx    # pipe‑separated file (encrypted when enabled)

src/                     # source code (modular: compiler, service, storage, types, config, error)
```

---

## Encryption

Encryption is **disabled by default** in the current build.  
To enable full encryption with `age`:

1. Uncomment `age-crypto` and `age-setup` in `Cargo.toml`.
2. Wrap `read_table` and `write_table` in `src/lib/storage/` with `decrypt_with_passphrase` / `encrypt_with_passphrase`.

All data will be encrypted before writing to disk and decrypted when read.

---

## Error Handling

All errors are explicit through the `NeuxError` enum (see `src/error.rs`).  
The CLI never panics – every failure returns a descriptive error message.

---

## License

MIT License – see [LICENSE](LICENSE) file.

---

## Author

neuxdotdev – [https://github.com/neuxdotdev](https://github.com/neuxdotdev)
