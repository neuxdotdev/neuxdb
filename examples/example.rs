use age_setup::build_keypair;
use neuxdb::{init, run, set_data_dir, set_secret_key, Result};
use std::fs;

fn main() -> Result<()> {
    println!("=== NeuxDb Final Secure Example ===\n");

    // 1. Setup Path & CLEANUP (Untuk memudahkan demo berulang)
    let db_path = "db/secure_final";
    let _ = fs::remove_dir_all(db_path); // Ignore error if not exists
    set_data_dir(db_path)?;
    init()?;

    // 2. Generate Key Pair (age_setup)
    println!("[KEY] Generating keys...");
    let keypair = build_keypair().expect("Failed to generate keypair");
    println!("[KEY] Public: {}", keypair.public);

    // 3. Set Secret Key
    set_secret_key(&keypair.secret)?;

    // 4. Run SQL
    run("CREATE DATABASE bank")?;
    run("USE bank")?;
    run("CREATE TABLE nasabah (nama, phone)")?;

    println!("[DB] Inserting data...");
    run("INSERT INTO nasabah VALUES ('Budi', '08123456789')")?;
    run("INSERT INTO nasabah VALUES ('Ani', '08987654321')")?;

    println!("\n[RESULT]:");
    println!("{}", run("SELECT * FROM nasabah")?);

    // 5. Verify File Encryption
    let content = fs::read_to_string(format!("{}/bank/nasabah.nxdb", db_path)).unwrap_or_default();
    assert!(
        content.starts_with("-----BEGIN AGE ENCRYPTED FILE-----"),
        "File not encrypted!"
    );
    println!("\n✅ SUCCESS: Data is encrypted & integrity verified.");

    Ok(())
}
