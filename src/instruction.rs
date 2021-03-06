use crate::parser::{Token, TokenType};
#[derive(Debug, Clone)]
pub struct Instruction {
    pub ty: InstructionType,
    pub pc: u32,
    pub tokens: Vec<Token>,
}

impl Instruction {
    pub fn has_dest(&self) -> (bool, usize) {
        let mut i = 0;
        for t in &self.tokens {
            if t.ty == TokenType::Equal {
                return (true, i);
            } 
            i += 1;
        }
        (false, i)
    }

    pub fn has_jump(&self) -> (bool, usize) {
        let mut i = 0;
        for t in &self.tokens {
            if t.ty == TokenType::Semicolon {
                return (true, i);
            } 
            i += 1;
        }
        (false, i)
    }
}
#[derive(Debug, Clone)]
pub enum InstructionType {
    A,
    C,
    Label,
}

pub struct Pc {
    base: u32,
}

impl Pc {
    pub fn new() -> Self {
        Pc{base:0}
    }

    pub fn get(&mut self) -> u32 {
        self.base += 1;
        self.base
    }

    pub fn get_without_inc(&self) -> u32 {
        self.base + 1
    }
}