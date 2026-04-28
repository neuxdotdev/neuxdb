use crate::error::{DbError, Result};
use crate::parser::{Expr, Statement};
use crate::storage;
use crate::types::{Row, Schema, Value};
pub fn execute(stmt: Statement) -> Result<String> {
    match stmt {
        Statement::CreateTable { name, columns } => {
            storage::create_table(&name, columns)?;
            Ok(format!("Table '{}' created.", name))
        }
        Statement::DropTable(name) => {
            storage::drop_table(&name)?;
            Ok(format!("Table '{}' dropped.", name))
        }
        Statement::ShowTables => {
            let tables = storage::list_tables()?;
            if tables.is_empty() {
                return Ok("No tables found.".into());
            }
            Ok(tables.join("\n"))
        }
        Statement::Insert { table, values } => {
            storage::transact(&table, |schema, rows| {
                if values.len() != schema.columns.len() {
                    return Err(DbError::InvalidInput(format!(
                        "Column count mismatch: expected {}, got {}",
                        schema.columns.len(),
                        values.len()
                    )));
                }
                rows.push(values);
                Ok(())
            })?;
            Ok("1 row inserted.".into())
        }
        Statement::Select {
            columns,
            table,
            filter,
        } => {
            let (schema, headers, mut rows) = storage::read_only(&table)?;
            rows.retain(|r| eval_expr(r, &headers, filter.as_ref().unwrap_or(&Expr::True)));
            let result_rows = project_rows(&schema, headers, rows, columns)?;
            Ok(format_table(&result_rows.0, &result_rows.1))
        }
        Statement::Update {
            table,
            set_col,
            set_val,
            filter,
        } => {
            let mut count = 0;
            storage::transact(&table, |schema, rows| {
                let col_idx = schema
                    .columns
                    .iter()
                    .position(|c| c == &set_col)
                    .ok_or_else(|| DbError::ColumnNotFound(set_col.clone()))?;
                for row in rows.iter_mut() {
                    if eval_expr(row, &schema.columns, &filter) {
                        row[col_idx] = set_val.clone();
                        count += 1;
                    }
                }
                Ok(())
            })?;
            Ok(format!("{} rows updated.", count))
        }
        Statement::Delete { table, filter } => {
            let mut count = 0;
            storage::transact(&table, |schema, rows| {
                let original = rows.len();
                rows.retain(|r| !eval_expr(r, &schema.columns, &filter));
                count = original - rows.len();
                Ok(())
            })?;
            Ok(format!("{} rows deleted.", count))
        }
    }
}
fn project_rows(
    _schema: &Schema,
    headers: Vec<String>,
    rows: Vec<Row>,
    cols: Vec<String>,
) -> Result<(Vec<String>, Vec<Row>)> {
    let indices: Vec<usize> = if cols.len() == 1 && cols[0] == "*" {
        (0..headers.len()).collect()
    } else {
        cols.iter()
            .map(|c| {
                headers
                    .iter()
                    .position(|h| h == c)
                    .ok_or_else(|| DbError::ColumnNotFound(c.clone()))
            })
            .collect::<Result<Vec<usize>>>()?
    };
    let new_headers = indices.iter().map(|&i| headers[i].clone()).collect();
    let new_rows = rows
        .into_iter()
        .map(|row| indices.iter().map(|&i| row[i].clone()).collect())
        .collect();
    Ok((new_headers, new_rows))
}
fn eval_expr(row: &[Value], headers: &[String], expr: &Expr) -> bool {
    match expr {
        Expr::True => true,
        Expr::Eq(col, val) => compare(row, headers, col, val, |a, b| a == b),
        Expr::Ne(col, val) => compare(row, headers, col, val, |a, b| a != b),
        Expr::Gt(col, val) => compare(row, headers, col, val, |a, b| a > b),
        Expr::Ge(col, val) => compare(row, headers, col, val, |a, b| a >= b),
        Expr::Lt(col, val) => compare(row, headers, col, val, |a, b| a < b),
        Expr::Le(col, val) => compare(row, headers, col, val, |a, b| a <= b),
        Expr::Like(col, val) => match get_value(row, headers, col) {
            Some(cell) => like_match(&cell.to_string_cmp(), &val.to_string_cmp()),
            None => false,
        },
        Expr::And(a, b) => eval_expr(row, headers, a) && eval_expr(row, headers, b),
        Expr::Or(a, b) => eval_expr(row, headers, a) || eval_expr(row, headers, b),
    }
}
fn compare<F>(row: &[Value], headers: &[String], col: &str, val: &Value, cmp: F) -> bool
where
    F: Fn(&Value, &Value) -> bool,
{
    match get_value(row, headers, col) {
        Some(cell) => cmp(cell, val),
        None => false,
    }
}
fn get_value<'a>(row: &'a [Value], headers: &[String], col: &str) -> Option<&'a Value> {
    headers
        .iter()
        .position(|h| h == col)
        .and_then(|i| row.get(i))
}
fn like_match(s: &str, p: &str) -> bool {
    let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
    dp[0][0] = true;
    for j in 1..=p.len() {
        if p.as_bytes()[j - 1] == b'%' {
            dp[0][j] = dp[0][j - 1];
        }
    }
    let s_bytes = s.as_bytes();
    let p_bytes = p.as_bytes();
    for i in 1..=s.len() {
        for j in 1..=p.len() {
            if p_bytes[j - 1] == b'%' {
                dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
            } else if p_bytes[j - 1] == b'_' || p_bytes[j - 1] == s_bytes[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    dp[s.len()][p.len()]
}
fn format_table(headers: &[String], rows: &[Row]) -> String {
    let mut output = String::new();
    output.push_str(&headers.join(" | "));
    output.push('\n');
    output.push_str(&"-".repeat(headers.join(" | ").len()));
    output.push('\n');
    if rows.is_empty() {
        output.push_str("(0 rows)\n");
    } else {
        for row in rows {
            let line: Vec<String> = row.iter().map(|v| v.to_string()).collect();
            output.push_str(&line.join(" | "));
            output.push('\n');
        }
    }
    output
}
