use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use toml;

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub commands: Vec<Command>,
}

impl Config {
    pub fn new() -> Config {
        let commands: Vec<Command> = Vec::new();
        Config { commands }
    }
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = &self.description {
            write!(f, "{}: {}", self.key, description)
        } else {
            write!(f, "{}", self.key)
        }
    }
}

pub fn read() -> Result<Config, io::Error> {
    let home_dir = env::var("HOME").unwrap();
    let config_path = format!("{}/.cutlie.toml", home_dir);
    let mut file = File::open(&config_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    Ok(config)
}

pub fn write(config: &Config) -> Result<(), io::Error> {
    let home_dir = env::var("HOME").unwrap();
    let config_path = format!("{}/.cutlie.toml", home_dir);
    let contents = toml::to_string(config).unwrap();
    let mut file = File::create(&config_path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
    Ok(())
}
