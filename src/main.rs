use crate::{interpreter::Interpreter, parser::Parser};

mod parser;
mod interpreter;

fn main() {
    let mut parser = Parser::new(">!>d>l>r>o>w>1012>o>l>l>e>H!!!!!!!!!!!!".to_string());
    let instructions = parser.parse();
    println!("{:?}", instructions);

    let mut interpreter = Interpreter::new(instructions);
    interpreter.run();
}
