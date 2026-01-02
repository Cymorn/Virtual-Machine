use crate::instructions::Instruction;

pub fn parser_program(source: &str) -> Result<Vec<Instruction>, String> {

    let mut instructions = Vec::new();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "LOAD" => {
                let value = parts[1].parse::<i32>()
                    .map_err(|_| format!("Invalid LOAD value: {}", parts[1]))?;
                instructions.push(Instruction::Load(value));
            }

            "ADD" => {
                let value = parts[1].parse::<i32>()
                    .map_err(|_| format!("Invalid ADD value: {}", parts[1]))?;
                instructions.push(Instruction::Add(value));
            }

            "SUB" => {
                let value = parts[1].parse::<i32>()
                    .map_err(|_| format!("Invalid SUB value: {}", parts[1]))?;
                instructions.push(Instruction::Sub(value));
            }


            "PRINT" => {
                instructions.push(Instruction::Print);
            }

            "HALT" => {
                instructions.push(Instruction::Halt);
            }

            _ => {
                 return Err(format!("Unknown instruction: {}", line));
            }

        }
    }
    

    Ok(instructions)
}


