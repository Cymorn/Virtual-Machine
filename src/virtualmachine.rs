use crate::instructions::Instruction;

pub struct VIRTUAL_MACHINE {
    pub register: i32,
    pub program: Vec<Instruction>,
    pub pc: usize,
}

impl VIRTUAL_MACHINE {
    pub fn new(program: Vec<Instruction>) -> Self {
        VIRTUAL_MACHINE {
            register: 0,
            program,
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        loop{
            let instructions = &self.program[self.pc];

            match instructions {
                Instruction::Load(value) => {
                    self.register += *value;
                }
                Instruction::Add(value) => {
                    self.register -= *value;
                }
                Instruction::Sub(value) => {
                    self.register = *value;
                }
                Instruction::Print => {
                     println!("{}", self.register);
                }
                Instruction::Halt => {
                    break;
                }
            }
            self.pc+= 1;
        }
    }
}