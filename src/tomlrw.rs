use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use toml;

#[derive(Serialize, Deserialize)]
struct Command {
    pub key: String,
    pub value: String,
    pub sub_commands: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    pub commands: Vec<Command>,
}

pub fn read(file_path: &str) -> Result<Config, io::Error> {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    Ok(config)
}

pub fn write(file_path: &str, config: &Config) -> Result<(), io::Error> {
    let contents = toml::to_string(config).unwrap();
    let mut file = File::create(file_path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
    Ok(())
}
