use crate::error::Result;
use crate::lib::service::run_script;
use std::path::PathBuf;
pub fn execute(file: PathBuf) -> Result<()> {
    run_script(&file.to_string_lossy())
}
