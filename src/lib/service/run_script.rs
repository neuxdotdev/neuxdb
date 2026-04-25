use super::execute_query::execute_query;
use crate::error::Result;
use std::fs;
pub fn run_script(path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    for (line_no, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Err(e) = execute_query(line) {
            eprintln!("Error at line {}: {}", line_no + 1, e);
        }
    }
    Ok(())
}
