use std::{env, fs};

use crate::{interpreter::Interpreter, parser::Parser};

mod parser;
mod interpreter;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(s) => match s.as_str() {
            "run" => {
                let path = if let Some(s) = args.get(2) {
                    s.clone()
                } else {
                    String::from("main.tern")
                };

                let file = fs::read_to_string(&path);
                match file {
                    Ok(s) => {
                        let mut parser = Parser::new(s);
                        let instructions = parser.parse();

                        let mut interpreter = Interpreter::new(instructions);
                        interpreter.run();
                    },
                    Err(e) => eprintln!("Couldn't read file {}: {}", path, e),
                }
            },
            _ => eprintln!("Unknown command {}", s),
        },
        None => {
            println!("Ternlang {}\n", VERSION);
            println!("List of commands:");
            println!("Ternlang # displays this message");
            println!("Ternlang run <path> # runs <path> if it's a .tern file");
        }
    }
}
