use crate::config::table_path;
use crate::error::{NeuxError, Result};
use std::fs;
pub fn execute(table: String, force: bool) -> Result<()> {
    let path = table_path(&table)?;
    if !path.exists() {
        return Err(NeuxError::TableNotFound(table));
    }
    if !force {
        println!("Are you sure you want to drop table '{}'? [y/N]", table);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Drop cancelled.");
            return Ok(());
        }
    }
    fs::remove_file(path)?;
    println!("Table '{}' dropped.", table);
    Ok(())
}
