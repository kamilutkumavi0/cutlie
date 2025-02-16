use cutlie::parser;
use cutlie::runner;
use cutlie::tomlrw::{self, Command};
use dialoguer::Select;
use std::env;
use std::fs::File;
use std::io::Write;
use strsim::jaro_winkler;

fn main() {
    let args = parser::parse();

    let home_dir = if let Ok(env_var) = env::var("HOME") {
        env_var
    } else {
        eprintln!("Can't open home diroctary.");
        return;
    };
    let config_path = format!("{}/.cutlie.toml", home_dir);

    // Check if .cutlie.toml exists, create it if it doesn't
    if !std::path::Path::new(&config_path).exists() {
        let mut file = if let Ok(file_temp) = File::create(&config_path) {
            file_temp
        } else {
            return;
        };
        let default_config = tomlrw::Config::new();
        let contents = if let Ok(string_contents) = toml::to_string(&default_config) {
            string_contents
        } else {
            return;
        };
        if let Ok(_) = file.write_all(contents.as_bytes()) {
        } else {
            return;
        };
    }

    match args.command {
        parser::Commands::Add {
            name,
            value,
            description,
        } => {
            let mut config = tomlrw::read().unwrap_or(tomlrw::Config::new());
            let command = tomlrw::Command {
                key: name.clone(),
                value,
                description,
            };
            let mut checker = false;
            for command in &mut config.commands {
                if command.key == name {
                    checker = true;
                    break;
                }
            }
            if !checker {
                config.commands.push(command);
                if let Ok(_) = tomlrw::write(&config) {}
            }
        }
        parser::Commands::Delete { name } => {
            let mut config = tomlrw::read().unwrap_or(tomlrw::Config::new());
            config.commands.retain(|command| command.key != name);
            if let Ok(_) = tomlrw::write(&config) {}
        }
        parser::Commands::Update { name, value } => {
            let mut config = tomlrw::read().unwrap_or(tomlrw::Config::new());
            for command in &mut config.commands {
                if command.key == name {
                    command.value = value;
                    break;
                }
            }
            if let Ok(_) = tomlrw::write(&config) {}
        }
        parser::Commands::Run { name } => {
            let config = tomlrw::read().unwrap_or(tomlrw::Config::new());
            let mut sim_vec: Vec<&Command> = Vec::new();
            let mut checker_runner = false;
            for command in &config.commands {
                if command.key == name {
                    runner::run(&command.value);
                    checker_runner = true;
                } else if jaro_winkler(&command.key, &name) > 0.6 {
                    sim_vec.push(&command);
                }
            }
            if !checker_runner {
                let selection = Select::new()
                    .with_prompt("What do you choose?")
                    .items(&sim_vec)
                    .interact()
                    .unwrap();

                let selected = sim_vec[selection];
                for command in &config.commands {
                    if command.key == selected.key {
                        runner::run(&command.value);
                    }
                }
            }
        }
        parser::Commands::List => {
            let config = tomlrw::read().unwrap_or(tomlrw::Config::new());
            for command in &config.commands {
                println!("{}", command);
            }
        }
    }
}
