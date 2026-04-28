pub mod error;
mod executor;
mod parser;
mod storage;
mod types;
pub use error::{DbError, Result};
pub use types::{Row, Schema, Value};
pub fn run(sql: &str) -> Result<String> {
    let stmt = parser::parse(sql)?;
    executor::execute(stmt)
}
pub fn init() -> Result<()> {
    storage::init()
}
