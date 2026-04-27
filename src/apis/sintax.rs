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
