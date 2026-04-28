use crate::error::{DbError, Result};
use crate::syntax::{Expr, Statement};
use crate::types::Value;
pub fn parse(sql: &str) -> Result<Statement> {
    let tokens = tokenize(sql);
    let mut parser = Parser { tokens, pos: 0 };
    parser.parse_statement()
}
struct Parser {
    tokens: Vec<String>,
    pos: usize,
}
impl Parser {
    fn peek(&self) -> Option<&String> {
        self.tokens.get(self.pos)
    }
    fn consume(&mut self) -> Result<String> {
        self.tokens
            .get(self.pos)
            .cloned()
            .ok_or_else(|| DbError::Parse("Unexpected end of input".into()))
    }
    fn expect(&mut self, expected: &str) -> Result<()> {
        let tok = self.consume()?;
        if tok.to_lowercase() != expected.to_lowercase() {
            return Err(DbError::Parse(format!(
                "Expected '{}', found '{}'",
                expected, tok
            )));
        }
        self.pos += 1;
        Ok(())
    }
    fn parse_statement(&mut self) -> Result<Statement> {
        let cmd = self.consume()?.to_lowercase();
        self.pos += 1;
        match cmd.as_str() {
            "create" => self.parse_create(),
            "drop" => self.parse_drop(),
            "show" => self.parse_show(),
            "insert" => self.parse_insert(),
            "select" => self.parse_select(),
            "update" => self.parse_update(),
            "delete" => self.parse_delete(),
            "use" => self.parse_use(),
            "backup" => self.parse_backup(),
            "check" => self.parse_check(),
            _ => Err(DbError::Parse(format!("Unknown command: {}", cmd))),
        }
    }
    fn parse_create(&mut self) -> Result<Statement> {
        let target = self.consume()?.to_lowercase();
        self.pos += 1;
        match target.as_str() {
            "table" => {
                let name = self.consume()?;
                self.pos += 1;
                self.expect("(")?;
                let mut cols = Vec::new();
                loop {
                    let tok = self.consume()?;
                    if tok == ")" {
                        self.pos += 1;
                        break;
                    }
                    if tok == "," {
                        self.pos += 1;
                        continue;
                    }
                    cols.push(tok);
                    self.pos += 1;
                }
                Ok(Statement::CreateTable {
                    name,
                    columns: cols,
                })
            }
            "database" => {
                let name = self.consume()?;
                self.pos += 1;
                Ok(Statement::CreateDatabase(name))
            }
            _ => Err(DbError::Parse(
                "Expected 'TABLE' or 'DATABASE' after CREATE".into(),
            )),
        }
    }
    fn parse_drop(&mut self) -> Result<Statement> {
        let target = self.consume()?.to_lowercase();
        self.pos += 1;
        match target.as_str() {
            "table" => {
                let name = self.consume()?;
                self.pos += 1;
                Ok(Statement::DropTable(name))
            }
            "database" => {
                let name = self.consume()?;
                self.pos += 1;
                Ok(Statement::DropDatabase(name))
            }
            _ => Err(DbError::Parse(
                "Expected 'TABLE' or 'DATABASE' after DROP".into(),
            )),
        }
    }
    fn parse_show(&mut self) -> Result<Statement> {
        let target = self.consume()?.to_lowercase();
        self.pos += 1;
        match target.as_str() {
            "tables" => Ok(Statement::ShowTables),
            "databases" => Ok(Statement::ShowDatabases),
            _ => Err(DbError::Parse(
                "Expected 'TABLES' or 'DATABASES' after SHOW".into(),
            )),
        }
    }
    fn parse_use(&mut self) -> Result<Statement> {
        let name = self.consume()?;
        self.pos += 1;
        Ok(Statement::UseDatabase(name))
    }
    fn parse_backup(&mut self) -> Result<Statement> {
        self.expect("table")?;
        let name = self.consume()?;
        self.pos += 1;
        Ok(Statement::Backup { table: name })
    }
    fn parse_check(&mut self) -> Result<Statement> {
        self.expect("table")?;
        let name = self.consume()?;
        self.pos += 1;
        Ok(Statement::CheckIntegrity { table: name })
    }
    fn parse_insert(&mut self) -> Result<Statement> {
        self.expect("into")?;
        let table = self.consume()?;
        self.pos += 1;
        self.expect("values")?;
        self.expect("(")?;
        let mut vals = Vec::new();
        loop {
            let tok = self.consume()?;
            if tok == ")" {
                self.pos += 1;
                break;
            }
            if tok == "," {
                self.pos += 1;
                continue;
            }
            vals.push(parse_value_token(&tok));
            self.pos += 1;
        }
        Ok(Statement::Insert {
            table,
            values: vals,
        })
    }
    fn parse_select(&mut self) -> Result<Statement> {
        let mut cols = Vec::new();
        loop {
            let tok = self.consume()?;
            if tok.to_lowercase() == "from" {
                self.pos += 1;
                break;
            }
            cols.push(tok);
            self.pos += 1;
        }
        let table = self.consume()?;
        self.pos += 1;
        let filter = if self.peek().map(|s| s.to_lowercase()) == Some("where".to_string()) {
            self.pos += 1;
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(Statement::Select {
            columns: cols,
            table,
            filter,
        })
    }
    fn parse_update(&mut self) -> Result<Statement> {
        let table = self.consume()?;
        self.pos += 1;
        self.expect("set")?;
        let col = self.consume()?;
        self.pos += 1;
        self.expect("=")?;
        let raw_val = self.consume()?;
        self.pos += 1;
        let val = parse_value_token(&raw_val);
        self.expect("where")?;
        let filter = self.parse_expr()?;
        Ok(Statement::Update {
            table,
            set_col: col,
            set_val: val,
            filter,
        })
    }
    fn parse_delete(&mut self) -> Result<Statement> {
        self.expect("from")?;
        let table = self.consume()?;
        self.pos += 1;
        self.expect("where")?;
        let filter = self.parse_expr()?;
        Ok(Statement::Delete { table, filter })
    }
    fn parse_expr(&mut self) -> Result<Expr> {
        let left = self.parse_primary_expr()?;
        if let Some(op) = self.peek() {
            if op.to_lowercase() == "and" {
                self.pos += 1;
                let right = self.parse_expr()?;
                return Ok(Expr::And(Box::new(left), Box::new(right)));
            }
            if op.to_lowercase() == "or" {
                self.pos += 1;
                let right = self.parse_expr()?;
                return Ok(Expr::Or(Box::new(left), Box::new(right)));
            }
        }
        Ok(left)
    }
    fn parse_primary_expr(&mut self) -> Result<Expr> {
        let col = self.consume()?;
        self.pos += 1;
        let op = self.consume()?;
        self.pos += 1;
        let val = self.consume()?;
        self.pos += 1;
        let val = parse_value_token(&val);
        match op.as_str() {
            "=" => Ok(Expr::Eq(col, val)),
            "!=" | "<>" => Ok(Expr::Ne(col, val)),
            ">" => Ok(Expr::Gt(col, val)),
            ">=" => Ok(Expr::Ge(col, val)),
            "<" => Ok(Expr::Lt(col, val)),
            "<=" => Ok(Expr::Le(col, val)),
            s if s.to_lowercase() == "like" => Ok(Expr::Like(col, val)),
            _ => Err(DbError::Parse(format!("Unknown operator: {}", op))),
        }
    }
}
fn parse_value_token(s: &str) -> Value {
    if s.starts_with('\'') && s.ends_with('\'') {
        let inner = &s[1..s.len() - 1];
        Value::Text(inner.replace("''", "'"))
    } else {
        s.parse::<i64>()
            .map(Value::Int)
            .unwrap_or_else(|_| Value::Text(s.to_string()))
    }
}
fn tokenize(sql: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = sql.chars().peekable();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        if ch == '\'' {
            chars.next();
            let mut s = String::from("'");
            loop {
                match chars.next() {
                    Some('\'') => {
                        if chars.peek() == Some(&'\'') {
                            s.push_str("''");
                            chars.next();
                        } else {
                            s.push('\'');
                            break;
                        }
                    }
                    Some(c) => s.push(c),
                    None => break,
                }
            }
            tokens.push(s);
            continue;
        }
        if "!<>=|".contains(ch) {
            let mut op = String::new();
            while let Some(&c) = chars.peek() {
                if "!<>=|".contains(c) {
                    op.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(op);
            continue;
        }
        if "(),*;".contains(ch) {
            tokens.push(ch.to_string());
            chars.next();
            continue;
        }
        let mut word = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
                word.push(c);
                chars.next();
            } else {
                break;
            }
        }
        if !word.is_empty() {
            tokens.push(word);
        }
    }
    tokens
}
