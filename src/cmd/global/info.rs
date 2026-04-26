
use crate::config::{DATA_DIR, TABLE_EXT};
pub fn show_info() {
    println!("NeuxDB Information:");
    println!("  Version:     0.1.0");
    println!("  Data Dir:    {}", DATA_DIR);
    println!("  Table Ext:   .{}", TABLE_EXT);
    println!("  Delimiter:   '|'");
}