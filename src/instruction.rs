use std::rc::Rc;

use crate::parser::Token;
#[derive(Debug, Clone)]
pub struct Instruction {
    ty: InstructionType,
    pc: u32,
    tokens: Rc<Vec<Token>>,
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