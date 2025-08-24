use clap::{Parser, Subcommand};

/// CLI input parser with clap lib.
#[derive(Parser, Debug)]
#[command(name = "cutlie")]
#[command(about = "A short cut tool for specific commands")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// All subcommands for runner.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Adds a new command
    ///
    /// # Examples
    ///
    /// ```
    /// let args = parser::Cli::parse_from(&["cutlie", "add", "test", "--value", "echo test"]);
    /// if let parser::Commands::Add { name, value, description } = args.command {
    ///     assert_eq!(name, "test");
    ///     assert_eq!(value, "echo test");
    ///     assert_eq!(description, None);
    /// } else {
    ///     panic!("Expected Add command");
    /// }
    /// ```
    Add {
        name: String,
        #[arg(short, long)]
        value: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Deletes an existing command.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = parser::Cli::parse_from(&["cutlie", "delete", "test"]);
    /// if let parser::Commands::Delete { name } = args.command {
    ///     assert_eq!(name, "test");
    /// } else {
    ///     panic!("Expected Delete command");
    /// }
    /// ```
    Delete { name: String },
    /// Updates the value (sh command).
    ///
    /// # Examples
    ///
    /// ```
    /// let args = parser::Cli::parse_from(&["cutlie", "update", "test", "--value", "echo updated"]);
    /// if let parser::Commands::Update { name, value } = args.command {
    ///     assert_eq!(name, "test");
    ///     assert_eq!(value, "echo updated");
    /// } else {
    ///     panic!("Expected Update command");
    /// }
    /// ```
    Update {
        name: String,
        #[arg(short, long)]
        value: String,
    },
    /// Runs the value of the given key
    ///
    /// # Examples
    ///
    /// ```
    /// let args = parser::Cli::parse_from(&["cutlie", "run", "test"]);
    /// if let parser::Commands::Run { name } = args.command {
    ///     assert_eq!(name, "test");
    /// } else {
    ///     panic!("Expected Run command");
    /// }
    /// ```
    Run { name: String },
    /// Lists all commands with description.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = parser::Cli::parse_from(&["cutlie", "list"]);
    /// if let parser::Commands::List = args.command {
    ///     // Expected List command
    /// } else {
    ///     panic!("Expected List command");
    /// }
    /// ```
    List,
}

/// Parses the CLI arguments and returns the parsed `Cli` struct.
///
/// # Examples
///
/// ```
/// let args = parser::parse();
/// ```
pub fn parse() -> Cli {
    Cli::parse()
}
