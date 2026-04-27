use crate::error::{NeuxDbError, Result};
pub fn run_script(path: &str, callback: impl Fn(&str) -> Result<()>) -> Result<()> {
    let file = std::fs::File::open(path)?;
    use std::io::{BufRead, BufReader};
    let reader = BufReader::new(file);
    for (line_no, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        callback(line).map_err(|e| NeuxDbError::Parse(format!("Line {}: {}", line_no + 1, e)))?;
    }
    Ok(())
}
