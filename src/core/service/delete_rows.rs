use crate::core::storage::{read_table, write_table};
use crate::core::syntax::{ComparisonOp, WhereClause};
use crate::error::Result;
use crate::types::Value;
pub fn delete_rows(table: &str, condition: &WhereClause) -> Result<usize> {
    let (headers, rows) = read_table(table)?;
    let original_len = rows.len();
    let new_rows: Vec<_> = rows
        .into_iter()
        .filter(|row| !eval_where(row, &headers, condition))
        .collect();
    let deleted = original_len - new_rows.len();
    if deleted > 0 {
        write_table(table, &headers, &new_rows)?;
    }
    Ok(deleted)
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
