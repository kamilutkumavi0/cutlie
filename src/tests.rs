#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::runner;
    use crate::tomlrw::{self, Command};
    use clap::Parser;

    #[test]
    fn test_parser_add_command() {
        let args = parser::Cli::parse_from(&["cutlie", "add", "test", "--value", "echo test"]);
        if let parser::Commands::Add {
            name,
            value,
            description,
        } = args.command
        {
            assert_eq!(name, "test");
            assert_eq!(value, "echo test");
            assert_eq!(description, None);
        } else {
            panic!("Expected Add command");
        }
    }

    #[test]
    fn test_runner_run() {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg("echo test")
            .output()
            .expect("Failed to execute command");
        assert_eq!(String::from_utf8_lossy(&output.stdout), "test\n");
    }

    #[test]
    fn test_tomlrw_read_write() {
        let command = Command {
            key: "test".to_string(),
            value: "echo test".to_string(),
            description: Some("Test command".to_string()),
        };
        let mut config = tomlrw::Config::new();
        config.commands.push(command);

        let write_result = tomlrw::write(&config);
        assert!(write_result.is_ok());

        let read_config = tomlrw::read().unwrap();
        assert_eq!(read_config.commands.len(), 1);
        assert_eq!(read_config.commands[0].key, "test");
        assert_eq!(read_config.commands[0].value, "echo test");
        assert_eq!(
            read_config.commands[0].description,
            Some("Test command".to_string())
        );
    }
}
