use clap::Subcommand;
use std::path::PathBuf;
#[derive(Subcommand)]
pub enum Commands {
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
    Run {
        file: PathBuf,
    },
}
