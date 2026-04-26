use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser)]
#[command(
    name = "neuxdb",
    about = "Super simple encrypted database",
    version = "0.2.0",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: TopLevelCommands,
}
#[derive(Subcommand)]
pub enum TopLevelCommands {
    Database {
        #[command(subcommand)]
        cmd: DatabaseCommands,
    },
    Table {
        #[command(subcommand)]
        cmd: TableCommands,
    },
    Run {
        file: PathBuf,
    },
}
#[derive(Subcommand)]
pub enum DatabaseCommands {
    Create {
        #[arg(long, short)]
        path: Option<PathBuf>,
    },
    Info,
    Export {
        output: PathBuf,
        #[arg(long, short)]
        passphrase: Option<String>,
    },
    Import {
        input: PathBuf,
        #[arg(long, short)]
        passphrase: Option<String>,
    },
    Backup {
        name: Option<String>,
    },
    Restore {
        file: PathBuf,
    },
}
#[derive(Subcommand)]
pub enum TableCommands {
    Create {
        table: String,
        columns: Vec<String>,
    },
    Insert {
        table: String,
        values: Vec<String>,
    },
    Select {
        table: String,
        #[arg(default_value = "*")]
        columns: Vec<String>,
        #[arg(long, short)]
        r#where: Option<String>,
    },
    Update {
        table: String,
        #[arg(long)]
        set: String,
        #[arg(long, short)]
        r#where: String,
    },
    Delete {
        table: String,
        #[arg(long, short)]
        r#where: String,
    },
    Drop {
        table: String,
        #[arg(long, short)]
        force: bool,
    },
    List,
}
