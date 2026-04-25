#![allow(special_module_name)]
mod config;
mod error;
mod lib;
mod types;
use clap::Parser;
use error::Result;
use lib::service::{
    create_table, delete_rows, insert_row, parse_assignment, parse_condition, run_script,
    select_rows, update_rows,
};
use types::{cli::Cli, commands::Commands, Value};
fn main() -> Result<()> {
    config::ensure_data_dir()?;
    let cli = Cli::parse();
    match cli.command {
        Commands::Create { table, columns } => {
            create_table(&table, &columns)?;
        }
        Commands::Insert { table, values } => {
            let vals = values
                .into_iter()
                .map(|v| Value::from(v.as_str()))
                .collect();
            insert_row(&table, vals)?;
        }
        Commands::Select {
            table,
            columns,
            r#where,
        } => {
            let condition = if let Some(cond_str) = r#where {
                parse_condition(&cond_str)?
            } else {
                None
            };
            select_rows(&table, &columns, condition.as_ref())?;
        }
        Commands::Update {
            table,
            set,
            r#where,
        } => {
            let (set_col, set_val) = parse_assignment(&set)?;
            let (cond_col, cond_val) = parse_condition(&r#where)?.expect("WHERE clause required");
            update_rows(&table, &set_col, set_val, &(cond_col, cond_val))?;
        }
        Commands::Delete { table, r#where } => {
            let (col, val) = parse_condition(&r#where)?.expect("WHERE clause required");
            delete_rows(&table, &(col, val))?;
        }
        Commands::Run { file } => {
            run_script(&file.to_string_lossy())?;
        }
    }
    Ok(())
}
