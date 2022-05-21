// 1. loop label ins , set symbol
// 2. replace all vars.

use crate::{instruction::{Instruction, InstructionType}, symbols::{Symbol, Symbolable}, parser::TokenType};

pub fn write_bit(ins: Vec<Instruction>) -> String {
    let mut sym = Symbol::new();
    for i in &ins {
        match i.ty {
            InstructionType::Label => {
                if let TokenType::Ident(label) = &i.tokens.get(1).unwrap().ty{ 
                    sym.add(label, i.pc);
                }
                
            },
            _ => {}
        }
    }

    for i in &ins {
        match i.ty {
            InstructionType::A => {
                // loop all token , found first diffrent var
                for token in &i.tokens {
                    if let  TokenType::Ident(var) = &token.ty {
                        if !sym.has(var) {
                            let m = sym.get_var_mem();
                            sym.add(var, m);
                        }
                    }
                }
            },
            _ => {}
        }
    }
    println!("{:#?}", sym);
    String::from("")
}