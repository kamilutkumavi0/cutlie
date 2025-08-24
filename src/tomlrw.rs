#![doc = include_str!("../README.md")]

//! Toml read and writer for saving commands
//! 
//! This module provides functionality to read and write commands to a TOML configuration file.
//! It defines the `Command` and `Config` structs, and provides functions to read and write the configuration file.

use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use toml;

/// Command is struct for all commands that takes name as key
/// value as sh command
/// description for minimal info about what its do.
#[derive(Serialize, Deserialize)]
pub struct Command {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

/// All Commands stored in this struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub commands: Vec<Command>,
}

/// creats new Config for initialization.
impl Config {
    pub fn new() -> Config {
        let commands: Vec<Command> = Vec::new();
        Config { commands }
    }
}
/// Making Command printable
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = &self.description {
            write!(f, "{}: {}", self.key, description)
        } else {
            write!(f, "{}", self.key)
        }
    }
}

/// Reads all configs
///
/// This function reads the configuration file from the user's home directory and returns a `Config` struct.
/// If the file does not exist or cannot be read, it returns `None`.
///
/// # Examples
///
/// ```
/// let config = tomlrw::read();
/// if let Some(config) = config {
///     println!("Config read successfully");
/// } else {
///     println!("Failed to read config");
/// }
/// ```
pub fn read() -> Option<Config> {
    let home_dir = if let Ok(env_var) = env::var("HOME") {
        env_var
    } else {
        return None;
    };
    let config_path = format!("{}/.cutlie.toml", home_dir);
    let mut file = File::open(&config_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if let Ok(config) = toml::from_str(&contents) {
        Some(config)
    } else {
        None
    }
}

/// Returns Error not done in this stuation.
#[derive(Debug)]
pub enum WriteError {
    NotOpenHome,
    MissInfogiven,
    CantWriteOnFile,
}

/// Writes new config.
///
/// This function writes the given `Config` struct to the configuration file in the user's home directory.
/// If the file cannot be created or written to, it returns a `WriteError`.
///
/// # Examples
///
/// ```
/// let command = tomlrw::Command {
///     key: "test".to_string(),
///     value: "echo test".to_string(),
///     description: Some("Test command".to_string()),
/// };
/// let mut config = tomlrw::Config::new();
/// config.commands.push(command);
/// let result = tomlrw::write(&config);
/// if let Ok(_) = result {
///     println!("Config written successfully");
/// } else {
///     println!("Failed to write config");
/// }
/// ```
pub fn write(config: &Config) -> Result<(), WriteError> {
    let home_dir = if let Ok(env_var) = env::var("HOME") {
        env_var
    } else {
        return Err(WriteError::NotOpenHome);
    };
    let config_path = format!("{}/.cutlie.toml", home_dir);
    let contents = if let Ok(string_contents) = toml::to_string(config) {
        string_contents
    } else {
        return Err(WriteError::MissInfogiven);
    };
    let mut file = if let Ok(file_temp) = File::create(&config_path) {
        file_temp
    } else {
        return Err(WriteError::CantWriteOnFile);
    };
    if let Ok(_) = file.write_all(contents.as_bytes()) {
    } else {
        return Err(WriteError::CantWriteOnFile);
    };
    Ok(())
}
