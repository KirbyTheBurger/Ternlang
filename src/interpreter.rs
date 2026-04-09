use crate::parser::Instruction;

pub struct Interpreter {
    instructions: Vec<Instruction>,
    pos: usize,
    stack: Vec<u32>,
    term: console::Term,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Interpreter {
        Interpreter {
            instructions,
            pos: 0,
            stack: Vec::new(),
            term: console::Term::stdout(),
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
            Instruction::Push(v) => self.push_instr(*v),
            Instruction::Pop => { self.pop(); },
            Instruction::Print => self.print(),
            Instruction::GetInput => self.get_input(),
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

    fn get_input(&mut self) {
        match self.term.read_char() {
            Ok(c) => self.push(c as u32),
            Err(e) => eprintln!("Failed to get input: {}", e),
        }
    }

    fn push(&mut self, value: u32) {
        self.stack.push(value);
    }

    fn push_instr(&mut self, value: Instruction) {
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