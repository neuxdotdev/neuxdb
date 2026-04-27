use neuxdb::{self, *};
use std::env;
use std::fs;
use std::sync::Mutex;
static TEST_MUTEX: Mutex<()> = Mutex::new(());
fn setup(temp_dir: &str) {
    fs::create_dir_all(temp_dir).unwrap();
    env::set_var("NEUXDB_DATA_DIR", temp_dir);
}
fn teardown(temp_dir: &str) {
    let _ = fs::remove_dir_all(temp_dir);
    env::remove_var("NEUXDB_DATA_DIR");
}
#[test]
fn test_create_and_insert() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    ensure_data_dir().unwrap();
    create_table("users", &["id".into(), "name".into()]).unwrap();
    let vals = vec![Value::Text("1".into()), Value::Text("Alice".into())];
    insert_row("users", vals).unwrap();
    let (headers, rows) = read_table("users").unwrap();
    assert_eq!(headers, vec!["id", "name"]);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0][0], Value::Text("1".into()));
    assert_eq!(rows[0][1], Value::Text("Alice".into()));
    let rows = select_rows("users", &["*".into()], None).unwrap();
    assert_eq!(rows.len(), 1);
    teardown(&dir_path);
}
#[test]
fn test_select_with_condition() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    ensure_data_dir().unwrap();
    create_table("items", &["id".into(), "val".into()]).unwrap();
    insert_row(
        "items",
        vec![Value::Text("1".into()), Value::Text("foo".into())],
    )
    .unwrap();
    insert_row(
        "items",
        vec![Value::Text("2".into()), Value::Text("bar".into())],
    )
    .unwrap();
    let cond = WhereClause::Condition {
        column: "id".to_string(),
        operator: ComparisonOp::Eq,
        value: Value::Text("2".into()),
    };
    let rows = select_rows("items", &["val".into()], Some(&cond)).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0][0], Value::Text("bar".into()));
    teardown(&dir_path);
}
#[test]
fn test_update_and_delete() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    ensure_data_dir().unwrap();
    create_table("data", &["k".into(), "v".into()]).unwrap();
    insert_row(
        "data",
        vec![Value::Text("x".into()), Value::Text("10".into())],
    )
    .unwrap();
    insert_row(
        "data",
        vec![Value::Text("y".into()), Value::Text("20".into())],
    )
    .unwrap();
    let condition = WhereClause::Condition {
        column: "k".to_string(),
        operator: ComparisonOp::Eq,
        value: Value::Text("x".into()),
    };
    let updated = update_rows("data", "v", Value::Text("99".into()), &condition).unwrap();
    assert_eq!(updated, 1);
    let rows = select_rows("data", &["*".into()], None).unwrap();
    assert_eq!(rows[0][1], Value::Text("99".into()));
    let del_cond = WhereClause::Condition {
        column: "k".to_string(),
        operator: ComparisonOp::Eq,
        value: Value::Text("y".into()),
    };
    let deleted = delete_rows("data", &del_cond).unwrap();
    assert_eq!(deleted, 1);
    let rows = select_rows("data", &["*".into()], None).unwrap();
    assert_eq!(rows.len(), 1);
    teardown(&dir_path);
}
#[test]
fn test_display_empty() {
    let table = format_table(&["a".into(), "b".into()], &[]);
    assert!(table.contains("(no rows)"));
}
#[test]
fn test_error_table_not_found() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path().to_string_lossy().to_string();
    setup(&dir_path);
    ensure_data_dir().unwrap();
    let res = select_rows("ghost", &["*".into()], None);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(matches!(err, NeuxDbError::TableNotFound(_)));
    teardown(&dir_path);
}
