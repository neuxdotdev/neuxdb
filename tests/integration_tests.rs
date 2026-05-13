use neuxdb::*;
use std::fs;
fn temp_dir() -> tempfile::TempDir {
    tempfile::TempDir::new().expect("gagal membuat temp dir")
}
fn db_path(dir: &tempfile::TempDir) -> std::path::PathBuf {
    dir.path().join("test.ndbx")
}
const PASS: &str = "485fa47fd179c6146cd8a6d26dad6427dab88208a068d44b6e9c6ec5763d0a93";
#[test]
fn create_database_should_create_file() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let db = Database::create(&path, PASS).unwrap();
    assert!(path.exists());
    drop(db);
}
#[test]
fn create_existing_database_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    Database::create(&path, PASS).unwrap();
    let result = Database::create(&path, PASS);
    assert!(result.is_err());
}
#[test]
fn open_database_wrong_password_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    Database::create(&path, PASS).unwrap();
    let result = Database::open(&path, "wrongpass");
    assert!(matches!(result, Err(Error::InvalidPassword)));
}
#[test]
fn open_database_correct_password_should_succeed() {
    let dir = temp_dir();
    let path = db_path(&dir);
    Database::create(&path, PASS).unwrap();
    let db = Database::open(&path, PASS).unwrap();
    drop(db);
}
#[test]
fn create_table_simple() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table(
        "users",
        vec![("id", ColumnType::Int), ("name", ColumnType::Text)],
    )
    .unwrap();
    assert_eq!(db.list_tables(), vec!["users"]);
}
#[test]
fn create_table_duplicate_name_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("a", ColumnType::Int)]).unwrap();
    let result = db.create_table("t", vec![("b", ColumnType::Text)]);
    assert!(matches!(result, Err(Error::TableExists(_))));
}
#[test]
fn create_table_duplicate_column_name_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    let result = db.create_table("t", vec![("a", ColumnType::Int), ("a", ColumnType::Text)]);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Duplicate column name"));
}
#[test]
fn create_table_invalid_name_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    assert!(db.create_table("", vec![("a", ColumnType::Int)]).is_err());
    assert!(db
        .create_table("123abc", vec![("a", ColumnType::Int)])
        .is_err());
    assert!(db
        .create_table("a-b", vec![("a", ColumnType::Int)])
        .is_err());
}
#[test]
fn drop_table_should_remove() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("x", ColumnType::Int)]).unwrap();
    db.drop_table("t").unwrap();
    assert!(db.list_tables().is_empty());
}
#[test]
fn drop_nonexistent_table_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    assert!(matches!(
        db.drop_table("ghost"),
        Err(Error::TableNotFound(_))
    ));
}
#[test]
fn table_schema_returns_correct_types() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table(
        "data",
        vec![
            ("a", ColumnType::Int),
            ("b", ColumnType::Bool),
            ("c", ColumnType::Float),
        ],
    )
    .unwrap();
    let schema = db.table_schema("data").unwrap();
    assert_eq!(
        schema,
        vec![
            ("a".to_string(), ColumnType::Int),
            ("b".to_string(), ColumnType::Bool),
            ("c".to_string(), ColumnType::Float),
        ]
    );
}
#[test]
fn insert_and_select_all() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table(
        "t",
        vec![("id", ColumnType::Int), ("name", ColumnType::Text)],
    )
    .unwrap();
    db.insert("t", vec![Value::Int(1), Value::Text("Alice".into())])
        .unwrap();
    db.insert("t", vec![Value::Int(2), Value::Text("Bob".into())])
        .unwrap();
    let rows = db.select("t", None, None).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0][0], Value::Int(1));
    assert_eq!(rows[1][1], Value::Text("Bob".into()));
}
#[test]
fn insert_type_mismatch_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("x", ColumnType::Int)]).unwrap();
    let result = db.insert("t", vec![Value::Text("not int".into())]);
    assert!(matches!(result, Err(Error::TypeMismatch { .. })));
}
#[test]
fn select_with_projection() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table(
        "t",
        vec![
            ("id", ColumnType::Int),
            ("name", ColumnType::Text),
            ("age", ColumnType::Int),
        ],
    )
    .unwrap();
    db.insert("t", vec![1.into(), "Alice".into(), 30.into()])
        .unwrap();
    db.insert("t", vec![2.into(), "Bob".into(), 25.into()])
        .unwrap();
    let rows = db.select("t", Some(vec!["name", "age"]), None).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].len(), 2);
    assert_eq!(rows[0][0], Value::Text("Alice".into()));
    assert_eq!(rows[0][1], Value::Int(30));
}
#[test]
fn select_with_filter() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("val", ColumnType::Int)])
        .unwrap();
    for i in 0..10 {
        db.insert("t", vec![Value::Int(i)]).unwrap();
    }
    let rows = db
        .select(
            "t",
            None,
            Some(&|row| match &row[0] {
                Value::Int(v) => *v > 5,
                _ => false,
            }),
        )
        .unwrap();
    assert_eq!(rows.len(), 4);
}
#[test]
fn update_single_row() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table(
        "t",
        vec![("key", ColumnType::Int), ("val", ColumnType::Text)],
    )
    .unwrap();
    db.insert("t", vec![1.into(), "old".into()]).unwrap();
    db.insert("t", vec![2.into(), "keep".into()]).unwrap();
    let updated = db
        .update(
            "t",
            &|row| row[0] == Value::Int(1),
            "val",
            Value::Text("new".into()),
        )
        .unwrap();
    assert_eq!(updated, 1);
    let rows = db
        .select("t", None, Some(&|row| row[0] == Value::Int(1)))
        .unwrap();
    assert_eq!(rows[0][1], Value::Text("new".into()));
}
#[test]
fn update_type_mismatch_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("x", ColumnType::Int)]).unwrap();
    db.insert("t", vec![1.into()]).unwrap();
    let result = db.update("t", &|_| true, "x", Value::Text("no".into()));
    assert!(matches!(result, Err(Error::TypeMismatch { .. })));
}
#[test]
fn delete_rows() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("x", ColumnType::Int)]).unwrap();
    for i in 0..5 {
        db.insert("t", vec![Value::Int(i)]).unwrap();
    }
    let removed = db
        .delete("t", &|row| match &row[0] {
            Value::Int(v) => v % 2 == 0,
            _ => false,
        })
        .unwrap();
    assert_eq!(removed, 3);
    assert_eq!(db.select("t", None, None).unwrap().len(), 2);
}
#[test]
fn commit_and_reopen_should_preserve_data() {
    let dir = temp_dir();
    let path = db_path(&dir);
    {
        let mut db = Database::create(&path, PASS).unwrap();
        db.create_table("t", vec![("msg", ColumnType::Text)])
            .unwrap();
        db.insert("t", vec![Value::Text("hello".into())]).unwrap();
        db.commit().unwrap();
    }
    {
        let db = Database::open(&path, PASS).unwrap();
        let rows = db.select("t", None, None).unwrap();
        assert_eq!(rows[0][0], Value::Text("hello".into()));
    }
}
#[test]
fn database_file_is_encrypted() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("msg", ColumnType::Text)])
        .unwrap();
    db.insert("t", vec![Value::Text("secret".into())]).unwrap();
    db.commit().unwrap();
    drop(db);
    let raw = fs::read(&path).unwrap();
    assert!(!raw.windows(6).any(|w| w == b"secret"));
    assert!(!raw.windows(6).any(|w| w == b"NEUXDB"));
}
#[test]
fn tampered_file_should_fail_integrity_check() {
    let dir = temp_dir();
    let path = db_path(&dir);
    {
        let mut db = Database::create(&path, PASS).unwrap();
        db.create_table("t", vec![("x", ColumnType::Int)]).unwrap();
        db.insert("t", vec![1.into()]).unwrap();
        db.commit().unwrap();
    }
    let mut data = fs::read(&path).unwrap();
    if let Some(byte) = data.last_mut() {
        *byte ^= 0xff;
    }
    fs::write(&path, &data).unwrap();
    let result = Database::open(&path, PASS);
    assert!(result.is_err());
}
#[test]
fn two_instances_cannot_open_same_file() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let _db1 = Database::create(&path, PASS).unwrap();
    let result = Database::open(&path, PASS);
    assert!(matches!(result, Err(Error::DatabaseLocked)));
}
#[test]
fn logs_are_recorded() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("a", ColumnType::Int)]).unwrap();
    db.insert("t", vec![1.into()]).unwrap();
    let logs = db.logs();
    assert!(logs.iter().any(|l| l.action == "CREATE TABLE"));
    assert!(logs.iter().any(|l| l.action == "INSERT"));
}
#[test]
fn export_json_contains_data() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("x", ColumnType::Int)]).unwrap();
    db.insert("t", vec![42.into()]).unwrap();
    let json = db.export_json().unwrap();
    assert!(json.contains("\"t\""));
    assert!(json.contains("42"));
}
#[test]
fn insert_wrong_number_of_columns_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("a", ColumnType::Int), ("b", ColumnType::Text)])
        .unwrap();
    assert!(db.insert("t", vec![1.into()]).is_err());
    assert!(db
        .insert("t", vec![1.into(), "x".into(), 2.into()])
        .is_err());
}
#[test]
fn select_nonexistent_column_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    db.create_table("t", vec![("id", ColumnType::Int)]).unwrap();
    db.insert("t", vec![1.into()]).unwrap();
    let result = db.select("t", Some(vec!["missing"]), None);
    assert!(matches!(result, Err(Error::ColumnNotFound(_))));
}
#[test]
fn operations_on_nonexistent_table_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    let mut db = Database::create(&path, PASS).unwrap();
    assert!(matches!(
        db.insert("ghost", vec![]),
        Err(Error::TableNotFound(_))
    ));
    assert!(matches!(
        db.select("ghost", None, None),
        Err(Error::TableNotFound(_))
    ));
    assert!(matches!(
        db.update("ghost", &|_| true, "x", Value::Int(1)),
        Err(Error::TableNotFound(_))
    ));
    assert!(matches!(
        db.delete("ghost", &|_| true),
        Err(Error::TableNotFound(_))
    ));
}
#[test]
fn weak_passphrase_should_error() {
    let dir = temp_dir();
    let path = db_path(&dir);
    assert!(Database::create(&path, "short").is_err());
}
#[test]
fn file_extension_validation() {
    let dir = temp_dir();
    let path = dir.path().join("wrong.txt");
    assert!(Database::create(&path, PASS).is_err());
}
#[test]
fn cannot_open_symlink() {
    let dir = temp_dir();
    let real = db_path(&dir);
    let link = dir.path().join("link.ndbx");
    Database::create(&real, PASS).unwrap();
    std::os::unix::fs::symlink(&real, &link).unwrap();
    let result = Database::open(&link, PASS);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Symlinks are not allowed"));
}
