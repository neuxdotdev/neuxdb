use super::DATA_DIR;
use crate::error::Result;
pub fn ensure_data_dir() -> Result<()> {
    std::fs::create_dir_all(DATA_DIR)?;
    Ok(())
}
