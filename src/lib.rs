pub mod admin;
pub mod config;
pub mod error;
mod executor;
pub mod manager;
mod parser;
mod storage;
mod syntax;
mod types;
pub use config::set_data_dir;
pub use error::{DbError, Result};
pub use types::{Row, Schema, Value};
pub fn run(sql: &str) -> Result<String> {
    let stmt = parser::parse(sql)?;
    executor::execute(stmt)
}
pub fn init() -> Result<()> {
    std::fs::create_dir_all(config::get_base_path())?;
    Ok(())
}
