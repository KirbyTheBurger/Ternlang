#[derive(Debug, Clone)]
pub enum Instruction {
    Number(u32),
    Character(char),

    Push(Box<Instruction>),
    Pop,
    Print,
    GetInput,
    Compare,
    Duplicate,

    Loop(Vec<Instruction>),

    Add,
    Sub,
    Mul,
    Div,

    EOF
}

pub struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        loop {
            self.skip_whitespace();

            let c = self.current();
            if matches!(c, None) {
                break;
            }

            instructions.push(self.parse_expression(*c.unwrap()));
        }

        if !matches!(instructions.last(), Some(Instruction::EOF)) {
            instructions.push(Instruction::EOF);
        }

        instructions
    }

    fn parse_expression(&mut self, current: char) -> Instruction {
        match current {
            '>' => {
                self.advance();
                Instruction::Push(Box::new(self.read_value()))
            },
            '<' => self.adv_ret(Instruction::Pop),
            '!' => self.adv_ret(Instruction::Print),
            '?' => self.adv_ret(Instruction::GetInput),
            '+' => self.adv_ret(Instruction::Add),
            '-' => self.adv_ret(Instruction::Sub),
            '*' => self.adv_ret(Instruction::Mul),
            '/' => self.adv_ret(Instruction::Div),
            '=' => self.adv_ret(Instruction::Compare),
            '@' => self.adv_ret(Instruction::Duplicate),
            '[' =>self.read_loop(),
            _ => panic!("Parser error, couldnt parse instruction starting with {}", current),
        }
    }

    fn read_loop(&mut self) -> Instruction {
        self.advance();

        let mut instructions = Vec::new();
        loop {
            let current = self.current();
            if matches!(current, Some(']') | None) {
                break;
            }

            instructions.push(self.parse_expression(*current.unwrap()));
        }
        self.advance();

        Instruction::Loop(instructions)
    }

    fn read_value(&mut self) -> Instruction {
        let current = match self.current() {
            Some(c) => *c,
            None => return Instruction::EOF,
        };

        if matches!(current, '0' | '1' | '2') {
            self.read_number(current)
        } else {
            self.advance();
            Instruction::Character(current)
        }
    }

    fn read_number(&mut self, current: char) -> Instruction {
        let mut ternary = String::from(current);
        self.advance();

        while let Some(c) = self.current() {
            if !matches!(c, '0' | '1' | '2') {
                break;
            }

            ternary.push(*c);
            self.advance();
        }

        Instruction::Number(
            u32::from_str_radix(ternary.as_str(), 3).expect("Couldnt parse ternary")
        )
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if !c.is_whitespace() {
                break;
            }

            self.advance();
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&char> {
        self.chars.get(self.pos)
    }

    fn adv_ret(&mut self, instruction: Instruction) -> Instruction {
        self.advance();
        instruction
    }
}