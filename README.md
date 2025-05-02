# cutlie

Cutlie is a short cut tool for you.

## Installation

To install Cutlie, you need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

Once you have Rust and Cargo installed, you can install Cutlie by running the following command:

```sh
cargo install cutlie
```

## Usage

Cutlie provides a command-line interface for managing and running shortcuts for specific commands. Here are the available commands:

### Add a new command

```sh
cutlie add <name> --value <command> [--description <description>]
```

Example:

```sh
cutlie add myssh --value "ssh user@192.168.1.1" --description "SSH to my server"
```

### Delete a command

```sh
cutlie delete <name>
```

Example:

```sh
cutlie delete myssh
```

### Update a command

```sh
cutlie update <name> --value <new_command>
```

Example:

```sh
cutlie update myssh --value "ssh user@192.168.1.2"
```

### Run a command

```sh
cutlie run <name>
```

Example:

```sh
cutlie run myssh
```

### List all commands

```sh
cutlie list
```

Example:

```sh
cutlie list
```

## Dependencies

Cutlie depends on the following libraries:

- `clap` (version 4.5.28) with the "derive" feature
- `dialoguer` (version 0.11.0)
- `serde` (version 1.0.217) with the "derive" feature
- `strsim` (version 0.11.1)
- `toml` (version 0.8.20)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Project Structure

The project structure is as follows:

- `src/main.rs`: The main entry point of the application.
- `src/lib.rs`: Contains the module declarations for the project.
- `src/parser.rs`: Handles command-line argument parsing using the `clap` library.
- `src/runner.rs`: Executes the commands using the `std::process::Command` module.
- `src/tomlrw.rs`: Reads and writes the configuration file (`.cutlie.toml`) using the `toml` and `serde` libraries.

## Future Plans

We have exciting plans for the future development of Cutlie. Here are some of the upcoming features, improvements, and other plans for the project's development:

- **Enhanced Command Management**: We plan to add more advanced command management features, such as tagging commands, organizing them into categories, and providing search functionality.

- **Cross-Platform Support**: We aim to ensure that Cutlie works seamlessly on different operating systems, including Windows, macOS, and Linux.

- **Documentation and Tutorials**: We will create comprehensive documentation and tutorials to help users get the most out of Cutlie.

- **Community Involvement**: We encourage community involvement and plan to actively engage with users, gather feedback, and incorporate their suggestions into future releases.

Stay tuned for updates and new releases as we continue to improve and expand Cutlie!

## Contributing

We welcome contributions to Cutlie! If you would like to contribute, please follow these guidelines:

1. Fork the repository and create a new branch for your feature or bug fix.
2. Write clear and concise commit messages.
3. Ensure that your code follows the project's coding style and conventions.
4. Write tests for your changes and ensure that all existing tests pass.
5. Submit a pull request with a detailed description of your changes.

## Contact

If you have any questions, suggestions, or feedback, please feel free to contact the project maintainer:

- Name: Kamil Utku Mavi
- Email: kamilutkumavi0@example.com
