use age_setup::build_keypair;
use neuxdb::{
    init, run, set_data_dir, set_secret_key, DbError, Result,
};
use std::fs;
use std::sync::Mutex;
static TEST_MUTEX: Mutex<()> = Mutex::new(());
fn setup(temp_dir: &str) {
    fs::create_dir_all(temp_dir).unwrap();
    std::env::set_var("NEUXDB_DATA_DIR", temp_dir);
    init().unwrap();
}
fn teardown(temp_dir: &str) {
    let _ = fs::remove_dir_all(temp_dir);
    std::env::remove_var("NEUXDB_DATA_DIR");
}
#[test]
fn test_secure_cycle() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let keypair = build_keypair().expect("Failed to generate keypair");
    set_secret_key(&keypair.secret)?;
    run("CREATE TABLE secrets (id, data)")?;
    run("INSERT INTO secrets VALUES (1, 'Classified')")?;
    let file_path = format!("{}/secrets.nxdb", dir_path);
    let content = fs::read_to_string(&file_path).expect("Failed to read file");
    assert!(content.starts_with("-----BEGIN AGE ENCRYPTED FILE-----"), "Data must be encrypted");
    assert!(!content.contains("Classified"), "Plaintext must not exist");
    let output = run("SELECT * FROM secrets")?;
    assert!(output.contains("Classified"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_integrity_check() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let keypair = build_keypair().unwrap();
    set_secret_key(&keypair.secret)?;
    run("CREATE TABLE tamper (val)")?;
    run("INSERT INTO tamper VALUES ('original')")?;
    let file_path = format!("{}/tamper.nxdb", dir_path);
    let mut content = fs::read_to_string(&file_path)?;
    let mid = content.len() / 2;
    let mut chars: Vec<char> = content.chars().collect();
    if chars[mid] == 'A' { chars[mid] = 'B'; } else { chars[mid] = 'A'; }
    content = chars.into_iter().collect();
    fs::write(&file_path, content)?;
    let res = run("SELECT * FROM tamper");
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(
        err.to_string().contains("Integrity check failed"),
        "Expected integrity error, got: {}", err
    );
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_wrong_key() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let key_a = build_keypair().unwrap();
    set_secret_key(&key_a.secret)?;
    run("CREATE TABLE secure (id)")?;
    run("INSERT INTO secure VALUES (99)")?;
    let key_b = build_keypair().unwrap();
    set_secret_key(&key_b.secret)?;
    let res = run("SELECT * FROM secure");
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(
        err.to_string().contains("Decryption failed"),
        "Expected decryption error, got: {}", err
    );
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_crud_operations() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let kp = build_keypair().unwrap();
    set_secret_key(&kp.secret)?;
    run("CREATE TABLE users (id, name)")?;
    run("INSERT INTO users VALUES (1, 'Alice')")?;
    run("INSERT INTO users VALUES (2, 'Bob')")?;
    let out = run("SELECT * FROM users")?;
    assert!(out.contains("Alice"));
    assert!(out.contains("Bob"));
    run("UPDATE users SET name = 'Alicia' WHERE id = 1")?;
    let out = run("SELECT * FROM users WHERE id = 1")?;
    assert!(out.contains("Alicia"));
    assert!(!out.contains("Alice"));
    run("DELETE FROM users WHERE id = 2")?;
    let out = run("SELECT * FROM users")?;
    assert!(!out.contains("Bob"));
    teardown(&dir_path);
    Ok(())
}
#[test]
fn test_db_management() -> Result<()> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    let kp = build_keypair().unwrap();
    set_secret_key(&kp.secret)?;
    run("CREATE DATABASE mydb")?;
    let dbs = run("SHOW DATABASES")?;
    assert!(dbs.contains("mydb"));
    run("USE mydb")?;
    run("CREATE TABLE data (x)")?;
    run("INSERT INTO data VALUES (100)")?;
    assert!(run("SELECT * FROM data")?.contains("100"));
    teardown(&dir_path);
    Ok(())
}