// 1. loop label ins , set symbol
// 2. replace all vars.

use std::collections::HashMap;

use crate::{instruction::{Instruction, InstructionType}, symbols::{Symbol, Symbolable}, parser::TokenType};

pub fn write_bit(ins: Vec<Instruction>) -> String {
    let mut sym = Symbol::new();
  for i in &ins {
        match i.ty {
            InstructionType::Label => {
                if let TokenType::Ident(label) = &i.tokens.get(1).unwrap().ty{ 
                    sym.add(label, i.pc);
                }
                
            }
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

    for i in &ins {
        match i.ty {
            InstructionType::A => {
                // replace var to address
                for token in &i.tokens {
                    if let  TokenType::Ident(var) = &token.ty {
                        let address = sym.get(var);
                    }
                }
            },
            _ => {}
        }
    }

    for i in &ins {
        match i.ty {
            InstructionType::A => {
                
            },
            InstructionType::C => {
                if i.has_dest() {
                    //first is dest
                }
                
                if i.has_jump() {
                    // last is jump
                } 
                // must has comp
                // loop all token, found comp tokens
                // between = ; 

                
            },
            _ => {}
        }
    }
    String::from("")
}

fn comp_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("0".into(), "0101010".into());
    map.insert("1".into(), "0111111".into());
    map.insert("-1".into(), "0111010".into());
    map.insert("D".into(), "0001100".into());
    map.insert("A".into(), "0110000".into());
    map.insert("!D".into(), "0001101".into());
    map.insert("!A".into(), "0110001".into());
    map.insert("-D".into(), "0001111".into());
    map.insert("-A".into(), "0110011".into());
    map.insert("D+1".into(), "0011111".into());
    map.insert("A+1".into(), "0110111".into());
    map.insert("D-1".into(), "0001110".into());
    map.insert("A-1".into(), "0110010".into());
    map.insert("D+A".into(), "0000010".into());
    map.insert("D-A".into(), "0010011".into());
    map.insert("A-D".into(), "0000111".into());
    map.insert("D&A".into(), "0000000".into());
    map.insert("D|A".into(), "0010101".into());
   
    map.insert("M".into(), "1110000".into());
    map.insert("!M".into(), "1110001".into());
    map.insert("-M".into(), "1110011".into());
    map.insert("M+1".into(), "1110111".into());
    map.insert("M-1".into(), "1110010".into());
    map.insert("D+M".into(), "1000010".into());
    map.insert("D-M".into(), "1010011".into());
    map.insert("D&M".into(), "1000000".into());
    map.insert("D|M".into(), "1010101".into());
    map
}

fn jump_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("JGT".into(), "001".into());
    map.insert("JEQ".into(), "010".into());
    map.insert("JGE".into(), "011".into());
    map.insert("JLT".into(), "100".into());
    map.insert("JNE".into(), "101".into());
    map.insert("JLE".into(), "110".into());
    map.insert("JMP".into(), "111".into());
    map
}

fn dest_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("M".into(), "001".into());
    map.insert("D".into(), "010".into());
    map.insert("MD".into(), "011".into());
    map.insert("A".into(), "100".into());
    map.insert("AM".into(), "101".into());
    map.insert("AD".into(), "110".into());
    map.insert("AMD".into(), "111".into());
    map
}