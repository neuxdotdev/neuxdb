use crate::error::Result;
use std::path::PathBuf;
pub fn execute(output: PathBuf, _passphrase: Option<String>) -> Result<()> {
    println!("Export to {} (not yet implemented)", output.display());
    Ok(())
}
