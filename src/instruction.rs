use crate::parser::Token;
#[derive(Debug, Clone)]
pub struct Instruction {
    pub ty: InstructionType,
    pub pc: u32,
    pub tokens: Vec<Token>,
}

impl Instruction {
    pub fn has_dest() -> bool {
        false
    }

    pub fn has_jump() -> bool {
        false
    }
}
#[derive(Debug, Clone)]
pub enum InstructionType {
    A,
    C,
}