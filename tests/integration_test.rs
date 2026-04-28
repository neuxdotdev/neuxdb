use neuxdb::{self, init, run, set_data_dir, DbError, Result};
use std::env;
use std::fs;
use std::sync::Mutex;
static TEST_MUTEX: Mutex<()> = Mutex::new(());
fn setup(temp_dir: &str) {
    fs::create_dir_all(temp_dir).unwrap();
    env::set_var("NEUXDB_DATA_DIR", temp_dir);
    set_data_dir(temp_dir).unwrap();
    init().unwrap();
}
fn teardown(temp_dir: &str) {
    let _ = fs::remove_dir_all(temp_dir);
    env::remove_var("NEUXDB_DATA_DIR");
}
#[test]
fn test_database_management() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let res = run("CREATE DATABASE my_app")?;
    assert!(res.contains("created"));
    let res = run("SHOW DATABASES")?;
    assert!(res.contains("my_app"));
    let res = run("USE my_app")?;
    assert!(res.contains("Switched"));
    run("CREATE TABLE config (key, val)")?;
    run("INSERT INTO config VALUES ('version', '1.0')")?;
    set_data_dir(&dir_path)?;
    let res = run("SELECT * FROM config");
    assert!(res.is_err(), "Table should not exist in root data dir");
    run("USE my_app")?;
    set_data_dir(&dir_path)?;
    let res = run("DROP DATABASE my_app")?;
    assert!(res.contains("dropped"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_admin_tools() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE TABLE users (id, name)")?;
    run("INSERT INTO users VALUES (1, 'Admin')")?;
    let res = run("BACKUP TABLE users")?;
    assert!(res.contains("backed up"));
    assert!(
        dir.path().join("users.nxdb.bak").exists(),
        "Backup file should exist"
    );
    let res = run("CHECK TABLE users")?;
    assert!(res.contains("OK"), "Table integrity should be OK");
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_create_insert_select() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let res = run("CREATE TABLE users (id, name)")?;
    assert!(res.contains("created"));
    let res = run("INSERT INTO users VALUES (1, 'Alice')")?;
    assert!(res.contains("1 row inserted"));
    let res = run("INSERT INTO users VALUES (2, 'Bob')")?;
    assert!(res.contains("1 row inserted"));
    let res = run("SELECT * FROM users")?;
    assert!(res.contains("Alice"));
    assert!(res.contains("Bob"));
    let res = run("SELECT name FROM users WHERE id = 2")?;
    assert!(res.contains("Bob"));
    assert!(!res.contains("Alice"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_update_delete() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE TABLE items (id, val)")?;
    run("INSERT INTO items VALUES (1, 'foo')")?;
    run("INSERT INTO items VALUES (2, 'bar')")?;
    let res = run("UPDATE items SET val = 'updated' WHERE id = 1")?;
    assert!(res.contains("1 rows updated"));
    let res = run("SELECT val FROM items WHERE id = 1")?;
    assert!(res.contains("updated"));
    let res = run("DELETE FROM items WHERE id = 2")?;
    assert!(res.contains("1 rows deleted"));
    let res = run("SELECT * FROM items")?;
    assert!(res.contains("updated"));
    assert!(!res.contains("bar"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_escape_quotes() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE TABLE logs (msg)")?;
    let res = run("INSERT INTO logs VALUES ('O''Hare')")?;
    assert!(res.contains("1 row inserted"));
    let res = run("SELECT * FROM logs")?;
    assert!(res.contains("O'Hare"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_show_tables() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE TABLE alpha (a)")?;
    run("CREATE TABLE beta (b)")?;
    let res = run("SHOW TABLES")?;
    assert!(res.contains("alpha"));
    assert!(res.contains("beta"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_drop_table() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE TABLE to_drop (x)")?;
    let res = run("DROP TABLE to_drop")?;
    assert!(res.contains("dropped"));
    let res = run("SELECT * FROM to_drop");
    assert!(res.is_err());
    if let Err(e) = res {
        assert!(matches!(e, DbError::TableNotFound(_)));
    }
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_errors() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let res = run("SELECT * FROM ghost");
    assert!(res.is_err());
    run("CREATE TABLE exists (a)")?;
    let res = run("CREATE TABLE exists (a)");
    assert!(res.is_err());
    if let Err(e) = res {
        assert!(matches!(e, DbError::TableExists(_)));
    }
    run("CREATE TABLE data (id)")?;
    let res = run("SELECT unknown FROM data");
    assert!(res.is_err());
    if let Err(e) = res {
        assert!(matches!(e, DbError::ColumnNotFound(_)));
    }
    let res = run("INSERT INTO data VALUES (1, 2)");
    assert!(res.is_err());
    if let Err(e) = res {
        assert!(matches!(e, DbError::InvalidInput(_)));
    }
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_like_operator() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE TABLE files (name)")?;
    run("INSERT INTO files VALUES ('report.pdf')")?;
    run("INSERT INTO files VALUES ('data.csv')")?;
    run("INSERT INTO files VALUES ('image.png')")?;
    let res = run("SELECT * FROM files WHERE name LIKE '%.pdf'")?;
    assert!(res.contains("report.pdf"));
    assert!(!res.contains("data.csv"));
    let res = run("SELECT * FROM files WHERE name LIKE 'data____'")?;
    assert!(res.contains("data.csv"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_concurrency_safety() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE TABLE counter (val)")?;
    run("INSERT INTO counter VALUES (0)")?;
    let mut handles = vec![];
    for _ in 0..5 {
        let path_clone = dir_path.clone();
        let handle = std::thread::spawn(move || {
            env::set_var("NEUXDB_DATA_DIR", &path_clone);
            set_data_dir(&path_clone).unwrap();
            for _ in 0..5 {
                let val_str = run("SELECT val FROM counter").unwrap();
                let current: i64 = val_str.lines().nth(2).unwrap().trim().parse().unwrap();
                let new_val = current + 1;
                let sql = format!(
                    "UPDATE counter SET val = {} WHERE val = {}",
                    new_val, current
                );
                let _ = run(&sql);
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    let res = run("SELECT * FROM counter")?;
    println!("Final counter state:\n{}", res);
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_manager_specific_errors() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE DATABASE db_test")?;
    let res = run("CREATE DATABASE db_test");
    assert!(res.is_err());
    if let Err(e) = res {
        let msg = e.to_string();
        assert!(
            msg.contains("already exists"),
            "Error message should contain 'already exists'"
        );
    }
    let res = run("DROP DATABASE ghost_db");
    assert!(res.is_err());
    if let Err(e) = res {
        let msg = e.to_string();
        assert!(msg.contains("not found"));
    }
    let res = run("USE DATABASE ghost_db");
    assert!(res.is_err());
    if let Err(e) = res {
        let msg = e.to_string();
        assert!(msg.contains("not found"));
    }
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_invalid_table_names() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let res = run("CREATE TABLE bad!table (id)");
    assert!(res.is_err());
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_parser_errors() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let res = run("FOO BAR");
    assert!(res.is_err());
    if let Err(e) = res {
        let msg = e.to_string();
        assert!(
            msg.contains("Unknown command"),
            "Expected unknown command error"
        );
    }
    let res = run("INSERT INTO users VALUES");
    assert!(res.is_err());
    run("CREATE TABLE t (a)")?;
    let res = run("SELECT * FROM t WHERE a <> 'val'");
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_string_parsing_edge_cases() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE TABLE texts (content)")?;
    run("INSERT INTO texts VALUES ('')")?;
    let res = run("SELECT * FROM texts")?;
    assert!(res.contains(""));
    run("INSERT INTO texts VALUES ('Hello World')")?;
    let res = run("SELECT * FROM texts WHERE content = 'Hello World'")?;
    assert!(res.contains("Hello World"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_value_type_coercion() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    run("CREATE TABLE nums (val)")?;
    run("INSERT INTO nums VALUES (100)")?;
    let res = run("SELECT * FROM nums WHERE val = 100")?;
    assert!(res.contains("100"));
    run("INSERT INTO nums VALUES ('200')")?;
    let res = run("SELECT * FROM nums WHERE val = 200")?;
    assert!(res.contains("200"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_error_messages_formatting() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let res = run("SELECT * FROM non_existent");
    if let Err(e) = res {
        let err_str = format!("{}", e);
        assert!(err_str.contains("not found"));
        let _ = format!("{:?}", e);
    }
    teardown(&dir_path);
    Ok(())
}
