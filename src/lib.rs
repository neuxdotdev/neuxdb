pub mod apis;
pub mod config;
pub mod error;
pub mod types;
pub use apis::compiler::parse;
pub use apis::service::{
    create_table, delete_rows, insert_row, parse_assignment, parse_condition, run_script,
    select_rows, update_rows,
};
pub use apis::sintax::Statement;
pub use apis::storage::{read_table, write_table};
pub use config::{
    delimiter_byte, delimiter_char, ensure_data_dir, sanitize_table_name, table_path,
};
pub use error::{NeuxError, Result};
pub use types::{Row, Value};
