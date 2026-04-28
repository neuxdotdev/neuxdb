use crate::types::Value;
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    CreateTable {
        name: String,
        columns: Vec<String>,
    },
    DropTable(String),
    CreateDatabase(String),
    DropDatabase(String),
    ShowDatabases,
    UseDatabase(String),
    ShowTables,
    Insert {
        table: String,
        values: Vec<Value>,
    },
    Select {
        columns: Vec<String>,
        table: String,
        filter: Option<Expr>,
    },
    Update {
        table: String,
        set_col: String,
        set_val: Value,
        filter: Expr,
    },
    Delete {
        table: String,
        filter: Expr,
    },
    Backup {
        table: String,
    },
    CheckIntegrity {
        table: String,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    True,
    Eq(String, Value),
    Ne(String, Value),
    Gt(String, Value),
    Ge(String, Value),
    Lt(String, Value),
    Le(String, Value),
    Like(String, Value),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}
