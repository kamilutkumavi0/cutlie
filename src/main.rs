use cutlie::runner;
use cutlie::parser;
use cutlie::tomlrw;
use std::collections::HashMap;

fn main() {
    let args = parser::parse();
    println!("Parsed command: {:?}", args.command);

    match args.command {
        parser::Commands::Add { name, sub } => {
            let mut config = tomlrw::read("config.toml").unwrap();
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
            tomlrw::write("config.toml", &config).unwrap();
        }
        parser::Commands::Delete { name } => {
            let mut config = tomlrw::read("config.toml").unwrap();
            config.commands.retain(|command| command.key != name);
            tomlrw::write("config.toml", &config).unwrap();
        }
        parser::Commands::Update { name, sub } => {
            let mut config = tomlrw::read("config.toml").unwrap();
            for command in &mut config.commands {
                if command.key == name {
                    if let Some(sub_command.clone()) = sub {
                        command.sub_commands.insert(sub_command.clone(), sub_command);
                    }
                }
            }
            tomlrw::write("config.toml", &config).unwrap();
        }
        parser::Commands::Run { name, sub } => {
            let config = tomlrw::read("config.toml").unwrap();
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
