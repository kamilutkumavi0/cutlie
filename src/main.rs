use cutlie::runner;
use cutlie::parser;
use cutlie::tomlrw;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // let deneme = tomlrw::Config::new();
    // tomlrw::write(".cutlie.toml", &deneme).unwrap();
    let args = parser::parse();
    println!("Parsed command: {:?}", args.command);

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
        parser::Commands::Add { name, value, sub } => {
            let mut config = tomlrw::read(&config_path).unwrap();
            let mut sub_commands = HashMap::new();
            if let Some(sub_command) = sub {
                sub_commands.insert(sub_command.clone(), sub_command);
            }
            let command = tomlrw::Command {
                key: name.clone(),
                value: name,
                sub_commands,
            };
            config.commands.push(command);
            tomlrw::write(&config_path, &config).unwrap();
        }
        parser::Commands::Delete { name } => {
            let mut config = tomlrw::read(&config_path).unwrap();
            config.commands.retain(|command| command.key != name);
            tomlrw::write(&config_path, &config).unwrap();
        }
        parser::Commands::Update { name, value, sub } => {
            let mut config = tomlrw::read(&config_path).unwrap();
            for command in &mut config.commands {
                if command.key == name {
                    if let Some(ref sub_command) = sub {
                        command.sub_commands.insert(sub_command.clone(), (*sub_command.clone()).to_string());
                    }
                }
            }
            tomlrw::write(&config_path, &config).unwrap();
        }
        parser::Commands::Run { name, sub } => {
            let config = tomlrw::read(&config_path).unwrap();
            for command in &config.commands {
                if command.key == name {
                    if let Some(ref sub_command) = sub {
                        if let Some(cmd) = command.sub_commands.get(sub_command) {
                            runner::run(cmd);
                        }
                    } else {
                        runner::run(&command.value);
                    }
                }
            }
        }
    }
}
