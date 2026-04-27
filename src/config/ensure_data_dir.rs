use crate::config::get_data_dir;
use crate::error::Result;
pub fn ensure_data_dir() -> Result<()> {
    std::fs::create_dir_all(get_data_dir())?;
    Ok(())
}
