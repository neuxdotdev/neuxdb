use super::execute_query::execute_query;
use crate::error::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn run_script(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for (line_no, line_result) in reader.lines().enumerate() {
        let line = line_result?;
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
