pub mod delimiter_byte;
pub mod delimiter_char;
pub mod ensure_data_dir;
pub mod sanitize_table_name;
pub mod schema_path;
pub mod table_path;
pub use delimiter_byte::delimiter_byte;
pub use delimiter_char::delimiter_char;
pub use ensure_data_dir::ensure_data_dir;
pub use sanitize_table_name::sanitize_table_name;
pub use schema_path::schema_path;
use std::env;
pub use table_path::table_path;
const DEFAULT_DATA_DIR: &str = "data";
pub(super) const DELIMITER: char = '|';
pub(super) const TABLE_EXT: &str = "nxdb";
pub(super) fn get_data_dir() -> String {
    env::var("NEUXDB_DATA_DIR").unwrap_or_else(|_| DEFAULT_DATA_DIR.to_string())
}
