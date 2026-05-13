use neuxdb::*;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
const PASS: &str = "strongpass123";
fn main() -> neuxdb::Result<()> {
    println!("╔══════════════════════════════════════════╗");
    println!("║        🔐 NeuxDB Full Demo               ║");
    println!("║   Embedded Encrypted Database Engine      ║");
    println!("╚══════════════════════════════════════════╝\n");
    let db_path = PathBuf::from("demo.ndbx");
    println!("[1/7] 🏗️  Membuat database baru...");
    if db_path.exists() {
        fs::remove_file(&db_path)?;
    }
    let mut db = Database::create(&db_path, PASS)?;
    println!("      ✅ Database dibuat di '{}'\n", db_path.display());
    println!("[2/7] 📥 Mengimpor data dari CSV...");
    let mut csv_data = Vec::new();
    fs::File::open("tmp/data.csv")
        .and_then(|mut f| f.read_to_end(&mut csv_data))
        .map_err(|e| Error::Io(e))?;
    db.import_table("devices", &csv_data, ExportFormat::Csv, false, None)?;
    println!("      ✅ Data CSV diimpor ke tabel 'devices'\n");
    println!("[3/7] 👀 Menampilkan 5 baris pertama:");
    let rows = db.select("devices", None, None)?;
    for (i, row) in rows.iter().take(5).enumerate() {
        println!("      Row {}: {:?}", i, row);
    }
    println!("      ... total {} baris\n", rows.len());
    println!("[4/7] 📤 Mengekspor tabel ke berbagai format...");
    let psv = db.export_table("devices", ExportFormat::Psv, false, None)?;
    fs::write("demo_export.psv", &psv)?;
    println!("      ✅ PSV -> demo_export.psv");
    let json = db.export_table("devices", ExportFormat::Json, false, None)?;
    fs::write("demo_export.json", &json)?;
    println!("      ✅ JSON -> demo_export.json");
    let html = db.export_table("devices", ExportFormat::Html, false, None)?;
    fs::write("demo_export.html", &html)?;
    println!("      ✅ HTML -> demo_export.html");
    let md = db.export_table("devices", ExportFormat::Markdown, false, None)?;
    fs::write("demo_export.md", &md)?;
    println!("      ✅ Markdown -> demo_export.md");
    let sql = db.export_table("devices", ExportFormat::SqliteDump, false, None)?;
    fs::write("demo_export.sql", &sql)?;
    println!("      ✅ SQLite dump -> demo_export.sql");
    let enc_csv = db.export_table("devices", ExportFormat::Csv, true, Some(PASS))?;
    fs::write("demo_export_enc.csv", &enc_csv)?;
    println!("      ✅ CSV terenkripsi -> demo_export_enc.csv\n");
    println!("[5/7] ✏️  Update: ubah 'merek' menjadi 'Samsung' untuk phone='123'");
    let updated = db.update(
        "devices",
        &|row| row[0] == Value::Text("123".into()),
        "merek",
        Value::Text("Samsung".into()),
    )?;
    println!("      {} baris diupdate\n", updated);
    println!("[6/7] 🗑️  Hapus data dengan tahun < 2020");
    let deleted = db.delete("devices", &|row| {
        if let Value::Int(y) = row[5] {
            y < 2020
        } else {
            false
        }
    })?;
    println!("      {} baris dihapus\n", deleted);
    println!("[7/7] 📜 Log transaksi:");
    for log in db.logs() {
        println!("      [{}] {} - {} {:?}", log.timestamp, log.action, log.table, log.details);
    }
    db.commit()?;
    println!("\n✅ Semua perubahan disimpan ke '{}'", db_path.display());
    println!("\n🔁 Membuka kembali database untuk verifikasi...");
    let db2 = Database::open(&db_path, PASS)?;
    let rows2 = db2.select("devices", None, None)?;
    println!("   Total rows: {}", rows2.len());
    println!("\n🎉 Demo selesai!");
    Ok(())
}