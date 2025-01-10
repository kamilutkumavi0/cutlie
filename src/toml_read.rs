use serde_derive::{Deserialize, Serialize};
use toml::from_str;
use home::home_dir;
use std::fs;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum ColorCutlie {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}


#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Command {
    name: String,
    logo: Option<String>,
    description: String,
    color: ColorCutlie,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Config {
    bg_color: ColorCutlie,
    commands: Vec<Command>,
}

pub fn toml_read(){
    let home_diroctory = home_dir();
    let config: Option<String> = match home_diroctory {
        Some(dir) => {
            let mut new_dir = dir.as_os_str().to_str().unwrap().to_string();
            new_dir.push_str("/.config/lh.toml");
            match fs::read_to_string(new_dir) {
                Ok(f) => Some(f),
                Err(_) => None,
            }
        }
        None => None,
    };
}
