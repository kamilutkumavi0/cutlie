use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cutlie")]
#[command(about = "A short cut tool for specific commands")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        sub: Option<String>,
    },
    Delete {
        #[arg(short, long)]
        name: String,
    },
    Update {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        sub: Option<String>,
    },
    Run {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        sub: Option<String>,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
