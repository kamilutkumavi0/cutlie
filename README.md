# cutlie
Cutlie is a short cut tool for spesific commands can be a like ssh command with ip adress

## Installation

To install Cutlie, you need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

Once you have Rust and Cargo installed, you can install Cutlie by running the following command:

```sh
cargo install cutlie
```

## Usage

Here are some examples of how to use Cutlie:

### Add a command

```sh
cutlie add --name "mycommand" --value "echo Hello, World!"
```

### Delete a command

```sh
cutlie delete --name "mycommand"
```

### Update a command

```sh
cutlie update --name "mycommand" --value "echo Hello, Cutlie!"
```

### Run a command

```sh
cutlie run --name "mycommand"
```

## Contributing

We welcome contributions to Cutlie! If you would like to contribute, please follow these guidelines:

1. Fork the repository and create a new branch for your feature or bugfix.
2. Write tests for your changes and ensure that all tests pass.
3. Submit a pull request with a clear description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
