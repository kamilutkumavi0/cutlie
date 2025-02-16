//! CLI parser for the binery.
use clap::{Parser, Subcommand};

/// Cli input parser with clap lib.
#[derive(Parser, Debug)]
#[command(name = "cutlie")]
#[command(about = "A short cut tool for specific commands")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// All sub commands for runner.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Adds new command
    Add {
        name: String,
        #[arg(short, long)]
        value: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Deletes existing command.
    Delete { name: String },
    /// updates the value (sh command).
    Update {
        name: String,
        #[arg(short, long)]
        value: String,
    },
    /// runs the value oh given key
    Run { name: String },
    /// Lists all cammands with description.
    List,
}

pub fn parse() -> Cli {
    Cli::parse()
}
