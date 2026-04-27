use crate::core::storage::{read_table, write_table};
use crate::core::syntax::{ComparisonOp, WhereClause};
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub fn update_rows(
    table: &str,
    set_col: &str,
    set_val: Value,
    condition: &WhereClause,
) -> Result<usize> {
    let (headers, mut rows) = read_table(table)?;
    let set_idx = headers
        .iter()
        .position(|c| c == set_col)
        .ok_or_else(|| NeuxDbError::ColumnNotFound(set_col.to_string(), table.to_string()))?;
    let mut updated = 0;
    for row in &mut rows {
        if eval_where(row, &headers, condition) {
            row[set_idx] = set_val.clone();
            updated += 1;
        }
    }
    if updated > 0 {
        write_table(table, &headers, &rows)?;
    }
    Ok(updated)
}
fn eval_where(row: &[Value], headers: &[String], clause: &WhereClause) -> bool {
    match clause {
        WhereClause::Condition {
            column,
            operator,
            value,
        } => {
            let idx = headers.iter().position(|h| h == column);
            if idx.is_none() {
                return false;
            }
            let cell = &row[idx.unwrap()];
            match operator {
                ComparisonOp::Eq => cell == value,
                ComparisonOp::Ne => cell != value,
                ComparisonOp::Lt => compare_values(cell, value) == std::cmp::Ordering::Less,
                ComparisonOp::Gt => compare_values(cell, value) == std::cmp::Ordering::Greater,
                ComparisonOp::Le => compare_values(cell, value) != std::cmp::Ordering::Greater,
                ComparisonOp::Ge => compare_values(cell, value) != std::cmp::Ordering::Less,
                ComparisonOp::Like => like_match(&cell.to_like_string(), &value.to_like_string()),
            }
        }
        WhereClause::And(a, b) => eval_where(row, headers, a) && eval_where(row, headers, b),
        WhereClause::Or(a, b) => eval_where(row, headers, a) || eval_where(row, headers, b),
    }
}
fn compare_values(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x.cmp(y),
        (Value::Text(x), Value::Text(y)) => x.cmp(y),
        _ => a.to_string().cmp(&b.to_string()),
    }
}
fn like_match(s: &str, pat: &str) -> bool {
    let mut s_iter = s.chars().peekable();
    let mut p_iter = pat.chars().peekable();
    loop {
        match p_iter.peek() {
            Some('%') => {
                p_iter.next();
                if p_iter.peek().is_none() {
                    return true;
                }
                while s_iter.peek().is_some() {
                    let rest_s: String = s_iter.clone().collect();
                    let rest_p: String = p_iter.clone().collect();
                    if like_match(&rest_s, &rest_p) {
                        return true;
                    }
                    s_iter.next();
                }
                return false;
            }
            Some('_') => {
                p_iter.next();
                if s_iter.next().is_none() {
                    return false;
                }
            }
            Some(&c) => {
                p_iter.next();
                if s_iter.next() != Some(c) {
                    return false;
                }
            }
            None => return s_iter.next().is_none(),
        }
    }
}
