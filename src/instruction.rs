use crate::parser::{Token, TokenType};
#[derive(Debug, Clone)]
pub struct Instruction {
    pub ty: InstructionType,
    pub pc: u32,
    pub tokens: Vec<Token>,
}

impl Instruction {
    pub fn has_dest(&self) -> bool {
        for t in &self.tokens {
            if t.ty == TokenType::Equal {
                return true;
            } 
        }
        false
    }

    pub fn has_jump(&self) -> bool {
        for t in &self.tokens {
            if t.ty == TokenType::Semicolon {
                return true;
            } 
        }
        false
    }
}
#[derive(Debug, Clone)]
pub enum InstructionType {
    A,
    C,
}