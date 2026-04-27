use crate::types::Value;
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    CreateTable {
        name: String,
        columns: Vec<String>,
    },
    DropTable {
        name: String,
    },
    ShowTables,
    Insert {
        table: String,
        values: Vec<Value>,
    },
    Select {
        columns: Vec<String>,
        table: String,
        condition: Option<WhereClause>,
    },
    Update {
        table: String,
        set_col: String,
        set_val: Value,
        condition: WhereClause,
    },
    Delete {
        table: String,
        condition: WhereClause,
    },
}
#[derive(Debug, PartialEq, Clone)]
pub enum WhereClause {
    Condition {
        column: String,
        operator: ComparisonOp,
        value: Value,
    },
    And(Box<WhereClause>, Box<WhereClause>),
    Or(Box<WhereClause>, Box<WhereClause>),
}
#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Like,
}
