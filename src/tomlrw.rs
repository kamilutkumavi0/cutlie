//! Toml read and writer for saving commands and issues
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

/// Issue is struct for tracking identified problems
#[derive(Serialize, Deserialize, Clone)]
pub struct Issue {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub severity: String,
    pub status: String,
    pub created_by: String,
}

/// All Commands and Issues stored in this struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub commands: Vec<Command>,
    pub issues: Vec<Issue>,
    pub next_issue_id: u32,
}

/// creates new Config for initialization.
impl Config {
    pub fn new() -> Config {
        let commands: Vec<Command> = Vec::new();
        let issues: Vec<Issue> = Vec::new();
        Config { 
            commands, 
            issues, 
            next_issue_id: 1 
        }
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

/// Making Issue printable
impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} [{}] {}: {}", 
               self.id, 
               self.severity, 
               self.title,
               self.status)
    }
}

/// Reads all configs
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
