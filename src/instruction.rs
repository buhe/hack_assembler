use std::rc::Rc;

use crate::parser::Token;
#[derive(Debug, Clone)]
pub struct Instruction<'a> {
    ty: InstructionType,
    pc: u32,
    tokens: Vec<&'a Token>,
}

impl Instruction<'_> {
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