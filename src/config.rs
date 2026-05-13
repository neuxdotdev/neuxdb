pub const DELIMITER: u8 = b'|';
pub const FILE_EXTENSION: &str = "ndbx";
pub const MAX_TABLE_NAME_LEN: usize = 64;
pub const MAX_COLUMN_NAME_LEN: usize = 64;
pub const MAX_COLUMNS_PER_TABLE: usize = 100;
pub const MAX_LOG_ENTRIES: usize = 10_000;
pub const MIN_PASSPHRASE_LEN: usize = 8;
pub const ALLOWED_NAME_REGEX: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*$";
pub const DB_VERSION: u8 = env!("CARGO_PKG_VERSION_MAJOR").as_bytes()[0] - b'0';
pub fn header_magic(version: u8) -> String {
    format!("[NEUXDB:v{}]", version)
}
pub const HMAC_PREFIX: &str = "HMAC=";
