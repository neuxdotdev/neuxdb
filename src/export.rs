use crate::error::{Error, Result};
use crate::table::TableData;
use crate::types::{ExportFormat, Value};
use crate::crypto;
pub fn export_table(
    table: &TableData,
    format: ExportFormat,
    encrypt: bool,
    passphrase: Option<&str>,
) -> Result<Vec<u8>> {
    let raw = match format {
        ExportFormat::Csv => to_csv(table),
        ExportFormat::Psv => to_psv(table),
        ExportFormat::Json => to_json(table),
        ExportFormat::Html => to_html(table),
        ExportFormat::Markdown => to_markdown(table),
        ExportFormat::SqliteDump => to_sqlite_dump(table),
    }?;
    if encrypt {
        let pass = passphrase
            .ok_or_else(|| Error::InvalidInput("Passphrase required for encryption".into()))?;
        let raw_str = String::from_utf8(raw.clone())
            .map_err(|_| Error::InvalidInput("Export produced non-UTF8 data".into()))?;
        crypto::encrypt(&raw_str, pass)
    } else {
        Ok(raw)
    }
}
fn to_csv(table: &TableData) -> Result<Vec<u8>> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(
        table
            .schema
            .columns
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<_>>(),
    )?;
    for row in &table.rows {
        let rec: Vec<String> = row.iter().map(|v| v.to_string()).collect();
        wtr.write_record(&rec)?;
    }
    wtr.flush()?;
    Ok(wtr
        .into_inner()
        .map_err(|e| Error::Csv(csv::Error::from(e.into_error())))?)
}
fn to_psv(table: &TableData) -> Result<Vec<u8>> {
    let delim = b'|';
    let mut out = vec![];
    let header = table
        .schema
        .columns
        .iter()
        .map(|c| c.name.as_str())
        .collect::<Vec<_>>()
        .join(&String::from_utf8_lossy(&[delim]));
    out.extend_from_slice(header.as_bytes());
    out.push(b'\n');
    for row in &table.rows {
        let line = row
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(&String::from_utf8_lossy(&[delim]));
        out.extend_from_slice(line.as_bytes());
        out.push(b'\n');
    }
    Ok(out)
}
fn to_json(table: &TableData) -> Result<Vec<u8>> {
    let rows: Vec<serde_json::Value> = table
        .rows
        .iter()
        .map(|row| {
            let mut obj = serde_json::Map::new();
            for (i, val) in row.iter().enumerate() {
                let key = &table.schema.columns[i].name;
                let json_val = match val {
                    Value::Int(i) => serde_json::Value::Number((*i).into()),
                    Value::Text(s) => serde_json::Value::String(s.clone()),
                    Value::Bool(b) => serde_json::Value::Bool(*b),
                    Value::Float(f) => serde_json::Value::Number(
                        serde_json::Number::from_f64(*f).unwrap_or(0.into()),
                    ),
                    Value::Null => serde_json::Value::Null,
                };
                obj.insert(key.clone(), json_val);
            }
            serde_json::Value::Object(obj)
        })
        .collect();
    let json = serde_json::to_string_pretty(&rows)?;
    Ok(json.into_bytes())
}
fn to_html(table: &TableData) -> Result<Vec<u8>> {
    let mut table_html = String::from("<table>\n<thead>\n<tr>");
    for col in &table.schema.columns {
        table_html.push_str(&format!("<th>{}</th>", escape_html(&col.name)));
    }
    table_html.push_str("</tr>\n</thead>\n<tbody>\n");
    for row in &table.rows {
        table_html.push_str("<tr>");
        for val in row {
            table_html.push_str(&format!("<td>{}</td>", escape_html(&val.to_string())));
        }
        table_html.push_str("</tr>\n");
    }
    table_html.push_str("</tbody>\n</table>");
    let template = include_str!("templates/export_template.html");
    let full_html = template
        .replace("{{table_name}}", &escape_html(&table.schema.name))
        .replace("{{table_content}}", &table_html);
    Ok(full_html.into_bytes())
}
fn to_markdown(table: &TableData) -> Result<Vec<u8>> {
    let mut md = String::new();
    let headers: Vec<String> = table.schema.columns.iter().map(|c| c.name.clone()).collect();
    md.push_str("| ");
    md.push_str(&headers.join(" | "));
    md.push_str(" |\n|");
    for _ in &headers {
        md.push_str(" --- |");
    }
    md.push('\n');
    for row in &table.rows {
        let line: Vec<String> = row.iter().map(|v| v.to_string()).collect();
        md.push_str("| ");
        md.push_str(&line.join(" | "));
        md.push_str(" |\n");
    }
    Ok(md.into_bytes())
}
fn to_sqlite_dump(table: &TableData) -> Result<Vec<u8>> {
    let mut dump = String::new();
    let table_name = &table.schema.name;
    let col_defs: Vec<String> = table
        .schema
        .columns
        .iter()
        .map(|c| {
            let sql_type = match c.col_type {
                crate::table::ColumnType::Int => "INTEGER",
                crate::table::ColumnType::Text => "TEXT",
                crate::table::ColumnType::Bool => "INTEGER",
                crate::table::ColumnType::Float => "REAL",
            };
            format!("\"{}\" {}", c.name, sql_type)
        })
        .collect();
    dump.push_str(&format!(
        "CREATE TABLE \"{}\" ({})",
        table_name,
        col_defs.join(", ")
    ));
    dump.push_str(");\n");
    for row in &table.rows {
        let vals: Vec<String> = row
            .iter()
            .map(|v| match v {
                Value::Null => "NULL".to_string(),
                Value::Int(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Bool(b) => if *b { "1".to_string() } else { "0".to_string() },
                Value::Text(s) => format!("'{}'", s.replace('\'', "''")),
            })
            .collect();
        dump.push_str(&format!(
            "INSERT INTO \"{}\" VALUES ({})",
            table_name,
            vals.join(", ")
        ));
        dump.push_str(");\n");
    }
    Ok(dump.into_bytes())
}
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}