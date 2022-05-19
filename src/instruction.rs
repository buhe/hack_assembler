use std::rc::Rc;

use crate::parser::Token;

struct Instruction {
    ty: InstructionType,
    pc: u32,
    tokens: Rc<Vec<Token>>,
}

enum InstructionType {
    A,
    C,
}