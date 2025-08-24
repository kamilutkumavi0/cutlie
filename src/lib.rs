/// This module contains the core functionality of the Cutlie application.
/// It includes the `runner`, `tomlrw`, and `parser` modules, as well as the test module.

pub mod runner; // The `runner` module handles the execution of commands.
pub mod tomlrw; // The `tomlrw` module handles reading and writing the configuration file.
pub mod parser; // The `parser` module handles command-line argument parsing.

#[cfg(test)]
mod tests; // The `tests` module contains test cases for the application.
