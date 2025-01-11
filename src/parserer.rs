use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cutlie")]
#[command(about = "A shortcut tool for specific commands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        description: String,
    },
    Run {
        name: String,
    },
    List,
    Select {
        name: String,
    },
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name, description } => {
            println!("Adding command: {} with description: {}", name, description);
            // Implement the logic for adding a command
        }
        Commands::Run { name } => {
            println!("Running command: {}", name);
            // Implement the logic for running a command
        }
        Commands::List => {
            println!("Listing all commands");
            // Implement the logic for listing all commands
        }
        Commands::Select { name } => {
            println!("Selecting command: {}", name);
            // Implement the logic for selecting a command
        }
    }
}
