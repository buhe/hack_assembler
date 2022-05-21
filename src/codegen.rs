// 1. loop label ins , set symbol
// 2. replace all vars.

use std::collections::HashMap;

use crate::{instruction::{Instruction, InstructionType}, symbols::{Symbol, Symbolable}, parser::TokenType};

pub fn write_bit(ins: Vec<Instruction>) -> String {
    let mut bit = String::new();
    let mut sym = Symbol::new();
    let dest_map = dest_map();
    let jump_map = jump_map();
    let comp_map = comp_map();
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
                        let replace = TokenType::Num(address);
                        // todo1
                    }
                }
            },
            _ => {}
        }
    }

    for i in &ins {
        match i.ty {
            InstructionType::A => {
                bit.push_str("0");
                match i.tokens.get(1).unwrap().ty {
                    TokenType::Num(n) => {
                        bit.push_str(format!("{:015}", n).as_str());
                    },
                    _ => panic!("expect number"),
                }
                bit.push_str("\n");
            },
            InstructionType::C => {
                let has_dest = i.has_dest();
                let has_jump = i.has_jump();
                bit.push_str("111");
                // must has comp
                // loop all token, found comp tokens
                // comp between = ; 
                let mut start_index:usize = 0;
                let mut end_index = i.tokens.len();
                if has_dest.0 {
                    start_index = has_dest.1 + 1;
                }
                if has_jump.0 {
                    end_index = has_jump.1;
                }
                let mut raw_comp = String::new();
                while end_index > start_index {
                    match &i.tokens.get(start_index).unwrap().ty {
                        TokenType::Num(num) => {
                            raw_comp.push_str(format!("{}", num).as_str());
                        },
                        TokenType::Neg => {
                            raw_comp.push_str("-");
                        },
                        TokenType::Add => {
                            raw_comp.push_str("+");
                        }
                        TokenType::Register(reg) => {
                            raw_comp.push_str(reg);
                        },
                        _ => {
                            panic!("error type");
                        }
                    }
                    start_index += 1;
                }
                // println!("raw {}", raw_comp);
                let comp = comp_map.get(&raw_comp).unwrap();
                bit.push_str(comp);
                if has_dest.0 {
                    //first is dest
                    match &i.tokens.first().unwrap().ty {
                        TokenType::Register(i) => {
                            let d = dest_map.get(i).unwrap();
                            bit.push_str(d);
                        },
                        _ => panic!("expect ident"),
                    }
                } else {
                    bit.push_str("000");
                }
                
                if has_jump.0 {
                    // last is jump
                      match &i.tokens.last().unwrap().ty {
                        TokenType::Jump(j) => {
                            let jump = jump_map.get(j).unwrap();
                            bit.push_str(jump);
                        },
                        _ => panic!("expect jump"),
                    }
                } else {
                    bit.push_str("000");
                }

                bit.push_str("\n");
             

                
            },
            _ => {}
        }
    }
    bit
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