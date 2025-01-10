use serde_derive::Deserialize;
use toml::from_str;
use home::home_dir;
use std::fs;

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



pub struct Command {
    name: String,
    logo: Option<String>,
    description: String,
    color: ColorCutlie,
}

pub struct Config {
    bg_color: ColorCutlie,
    commands: Vec<Command>,
}
