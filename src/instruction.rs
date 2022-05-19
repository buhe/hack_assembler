use std::rc::Rc;

use crate::parser::Token;

struct Instruction {
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

enum InstructionType {
    A,
    C,
}