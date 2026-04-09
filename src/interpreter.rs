use crate::parser::Instruction;

pub struct Interpreter {
    instructions: Vec<Instruction>,
    pos: usize,
    stack: Vec<u32>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Interpreter {
        Interpreter {
            instructions,
            pos: 0,
            stack: Vec::new(),
        }
    }

    pub fn display_stack(&self) {
        println!("{:?}", self.stack);
    }

    pub fn run(&mut self) {
        loop {
            let c = self.current().cloned();
            if matches!(c, Some(Instruction::EOF)) {
                break;
            }

            self.eval_instruction(c.expect("Expected instruction, found `None`"));
            self.advance();
        }
    }

    fn eval_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Push(v) => self.push(*v),
            Instruction::Pop => { self.pop(); },
            Instruction::Print => self.print(),
            Instruction::Error => eprintln!("Couldn't parse code"),
            _ => eprintln!("Unknown expression"),
        }
    }

    fn print(&mut self) {
        let value = self.pop();
        let char = char::from_u32(value);
        
        match char {
            Some(c) => print!("{c}"),
            None => eprintln!("{} is not a valid unicode character", value),
        }
    }

    fn push(&mut self, value: Instruction) {
        match value {
            Instruction::Number(n) => self.stack.push(n),
            Instruction::Character(c) => self.stack.push(c as u32),
            _ => eprintln!("Couldn't push instruction {:?} onto the stack, invalid type", value),
        }
    }

    fn pop(&mut self) -> u32 {
        self.stack.pop().expect("Can't pop if stack is empty")
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&Instruction> {
        self.instructions.get(self.pos)
    }
}