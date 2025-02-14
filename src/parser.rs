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
        name: String,
        #[arg(short, long)]
        value: String,
    },
    Delete {
        name: String,
    },
    Update {
        name: String,
        #[arg(short, long)]
        value: String,
    },
    Run {
        name: String,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
