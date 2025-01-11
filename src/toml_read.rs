use serde_derive::{Deserialize, Serialize};
use toml::from_str;
use home::home_dir;
use std::fs;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Command {
    name: String,
    logo: Option<String>,
    description: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Config {
    commands: Vec<Command>,
}

pub enum ConfigError{
    HomeDirNotFound,
    CutlieNotInitialize,
    ConfigTomlFormatError,
}

pub fn toml_read() -> Result<Config, ConfigError>{
    let home_diroctory = home_dir();
    let config: String = match home_diroctory {
        Some(dir) => {
            let mut new_dir = dir.as_os_str().to_str().unwrap().to_string();
            new_dir.push_str("/.config/cutlie/config.toml");
            match fs::read_to_string(new_dir) {
                Ok(f) => f,
                Err(_) => return Err(ConfigError::CutlieNotInitialize),
            }
        }
        None => return Err(ConfigError::HomeDirNotFound),
    };
    let config = match from_str(&config){
        Ok(f) => f,
        Err(e) => {
            dbg!(&e);
            return Err(ConfigError::ConfigTomlFormatError)},
    };
    Ok(config)
}
