use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cutlie")]
#[command(about = "A short cut tool for specific commands")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
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
