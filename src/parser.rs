use crate::instructions::Instruction;

pub fn parser_program(source: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "LOAD" => {
                let value: i32 = parts[1].parse().expect("Invalid LOAD value");
                instructions.push(Instruction::Load(value));
            }
            "ADD" => {
                let value: i32 = parts[1].parse().expect("Invalid ADD value");
                instructions.push(Instruction::Add(value));
            }

            "SUB" => {
                let value: i32 = parts[1].parse().expect("Invalid SUB value");
                instructions.push(Instruction::Sub(value));
            }

            "PRINT" => {
                instructions.push(Instruction::Print);
            }

            "HALT" => {
                instructions.push(Instruction::Halt);
            }

            _ => {
                panic!("Unknown instruction: {}", line);
            }
        }
    }

    instructions
}


