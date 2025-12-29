#[derive(Debug)]
pub enum Instruction {
    Load(i32),
    Add(i32),
    Sub(i32),
    Print,
    Halt,
}