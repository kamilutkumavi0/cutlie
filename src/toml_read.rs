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
