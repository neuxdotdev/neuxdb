use crate::error::Result;
use std::path::PathBuf;
pub fn execute(input: PathBuf, _passphrase: Option<String>) -> Result<()> {
    println!("Import from {} (not yet implemented)", input.display());
    Ok(())
}
