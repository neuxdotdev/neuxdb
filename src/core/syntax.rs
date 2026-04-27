use crate::types::Value;
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    CreateTable {
        name: String,
        columns: Vec<String>,
    },
    Insert {
        table: String,
        values: Vec<Value>,
    },
    Select {
        columns: Vec<String>,
        table: String,
        condition: Option<(String, Value)>,
    },
    Update {
        table: String,
        set_col: String,
        set_val: Value,
        condition: (String, Value),
    },
    Delete {
        table: String,
        condition: (String, Value),
    },
}
pub fn split_quoted(s: &str, delim: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\'' && !in_quote {
            in_quote = true;
            current.push(c);
        } else if c == '\'' && in_quote {
            in_quote = false;
            current.push(c);
        } else if c == delim && !in_quote {
            result.push(current.trim().to_string());
            current.clear();
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        result.push(current.trim().to_string());
    }
    result
}
