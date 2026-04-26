
pub fn show_help() {
    println!("NeuxDB v0.1.0 – Super simple encrypted database");
    println!();
    println!("Usage: neuxdb <COMMAND> [OPTIONS]");
    println!();
    println!("Commands:");
    println!("  create   Create a new table");
    println!("  insert   Insert a row into a table");
    println!("  select   Query rows from a table");
    println!("  update   Update rows in a table");
    println!("  delete   Delete rows from a table");
    println!("  run      Execute a .nxdb script file");
    println!();
    println!("Use 'neuxdb <command> --help' for more details.");
}