use crate::types::Row;
pub fn format_table(columns: &[String], rows: &[Row]) -> String {
    let header_strings: Vec<String> = columns.to_vec();
    let data_strings: Vec<Vec<String>> = rows
        .iter()
        .map(|row| row.iter().map(|v| v.to_string()).collect())
        .collect();
    let mut col_widths: Vec<usize> = header_strings.iter().map(|s| s.len()).collect();
    for row in &data_strings {
        for (i, cell) in row.iter().enumerate() {
            if i < col_widths.len() && cell.len() > col_widths[i] {
                col_widths[i] = cell.len();
            }
        }
    }
    let separator = || -> String {
        col_widths
            .iter()
            .map(|w| "-".repeat(*w))
            .collect::<Vec<_>>()
            .join("-+-")
    };
    let mut output = String::new();
    let header_line: Vec<String> = header_strings
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:width$}", h, width = col_widths[i]))
        .collect();
    output.push_str(&header_line.join(" | "));
    output.push('\n');
    output.push_str(&separator());
    output.push('\n');
    if rows.is_empty() {
        output.push_str("(no rows)\n");
    } else {
        for row in &data_strings {
            let line: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, cell)| format!("{:width$}", cell, width = col_widths[i]))
                .collect();
            output.push_str(&line.join(" | "));
            output.push('\n');
        }
    }
    output
}
