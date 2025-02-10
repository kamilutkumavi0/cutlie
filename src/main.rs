use cutlie::runner;
use cutlie::parser;

fn main() {
    let args = parser::parse();
    println!("Parsed command: {}", args.command);
}
