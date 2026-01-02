mod instructions;
mod virtualmachine;
mod parser;


use instructions::Instruction;
use virtualmachine::VIRTUAL_MACHINE;
use parser::parser_program;



use std::fs;

fn main() {
    let source = fs::read_to_string("program.vm")
        .expect("Failed to read program file");

    let instructions = parser_program(&source);

    let mut virtual_machine = virtualmachine::VIRTUAL_MACHINE::new(instructions);
    virtual_machine.run();
}
