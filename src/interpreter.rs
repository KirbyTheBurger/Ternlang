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

            self.eval_instruction(&c.expect("Expected instruction, found `None`"));
            self.advance();
        }
    }

    fn eval_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Push(v) => self.push_instr(*v.clone()),
            Instruction::Pop => { self.pop(); },
            Instruction::Print => self.print(),
            Instruction::GetInput => self.get_input(),
            Instruction::Error => eprintln!("Couldn't parse code"),
            Instruction::Add | Instruction::Sub | Instruction::Mul | Instruction::Div => {
                self.eval_arithmetic(instruction);
            },
            Instruction::Loop(i) => self.eval_loop(i),
            Instruction::Compare => self.eval_compare(),
            _ => eprintln!("Unknown expression"),
        }
    }

    fn eval_compare(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if a == b {
            self.push(1);
        } else if a < b {
            self.push(0);
        } else {
            self.push(2);
        }
    }

    fn eval_loop(&mut self, instructions: &Vec<Instruction>) {
        loop {
            let a = match self.stack.get(self.stack.len().wrapping_sub(1)) {
                Some(n) => n,
                None => return,
            };
            let b = match self.stack.get(self.stack.len().wrapping_sub(2)) {
                Some(n) => n,
                None => return,
            };
            if a == b { break; }

            for i in instructions {
                self.eval_instruction(i);
            }
        }
    }

    fn eval_arithmetic(&mut self, operation: &Instruction) {
        let b = self.pop();
        let a = self.pop();
        self.push(match operation {
            Instruction::Add => a + b,
            Instruction::Sub => a - b,
            Instruction::Mul => a * b,
            Instruction::Div => a / b,
            _ => panic!("Couldnt evaluate arithmetic operation, this is an inernal error. If this happens, please let me (the developer) know."),
        })
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