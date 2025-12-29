mod instructions;
mod virtualmachine;

use instructions::Instruction;
use virtualmachine::VIRTUAL_MACHINE;


fn main() {
    let program = vec![
        Instruction::Load(10),
        Instruction::Add(5),
        Instruction::Sub(3),
        Instruction::Print,
        Instruction::Halt,
    ];

    let mut virtualmachine = VIRTUAL_MACHINE::new(program);
    virtualmachine.run();
}