use cutlie::parser;
use cutlie::runner;
use cutlie::tomlrw::{self, Command};
use dialoguer::Select;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use strsim::jaro_winkler;

fn main() {
    // println!("The similarity between '{}' and '{}' is {}", string1, string2, similarity);
    let args = parser::parse();
    // println!("Parsed command: {:?}", args.command);

    let home_dir = env::var("HOME").unwrap();
    let config_path = format!("{}/.cutlie.toml", home_dir);

    // Check if .cutlie.toml exists, create it if it doesn't
    if !std::path::Path::new(&config_path).exists() {
        let mut file = File::create(&config_path).unwrap();
        let default_config = tomlrw::Config::new();
        let contents = toml::to_string(&default_config).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    match args.command {
        parser::Commands::Add { name, value } => {
            let mut config = tomlrw::read(&config_path).unwrap();
            let command = tomlrw::Command {
                key: name.clone(),
                value,
            };
            config.commands.push(command);
            tomlrw::write(&config_path, &config).unwrap();
        }
        parser::Commands::Delete { name } => {
            let mut config = tomlrw::read(&config_path).unwrap();
            config.commands.retain(|command| command.key != name);
            tomlrw::write(&config_path, &config).unwrap();
        }
        parser::Commands::Update { name, value } => {
            let mut config = tomlrw::read(&config_path).unwrap();
            for command in &mut config.commands {
                if command.key == name {}
            }
            tomlrw::write(&config_path, &config).unwrap();
        }
        parser::Commands::Run { name } => {
            let config = tomlrw::read(&config_path).unwrap();
            let mut sim_vec: Vec<String> = Vec::new();
            let mut checker_runner = false;
            for command in &config.commands {
                if command.key == name {
                    runner::run(&command.value);
                    checker_runner = true;
                } else if jaro_winkler(&command.key, &name) > 0.6 {
                    sim_vec.push(command.key.clone());
                }
            }
            if !checker_runner {
                let selection = Select::new()
                    .with_prompt("What do you choose?")
                    .items(&sim_vec)
                    .interact()
                    .unwrap();

                let selected = sim_vec[selection].clone();
                for command in &config.commands {
                    if command.key == selected {
                        runner::run(&command.value);
                    }
                }
            }
        }
    }
}
