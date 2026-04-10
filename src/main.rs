use crate::{interpreter::Interpreter, parser::Parser};

mod parser;
mod interpreter;

fn main() {
    let mut parser = Parser::new(">10>0[>1+>a!]".to_string());
    let instructions = parser.parse();
    println!("{:?}", instructions);

    let mut interpreter = Interpreter::new(instructions);
    interpreter.run();
}
