use std::rc::Rc;
use std::collections::HashMap;

use crate::{instruction::{Instruction, InstructionType, Pc}, symbols::Symbol};

pub fn tokenize(path: String) -> Vec<Instruction> {
    let mut tokenizer = Tokenizer::new(Rc::new(path));
    // println!("token: {:?}", tokenizer);
    tokenizer.scan(&keyword_map());
    println!("after scan token: {:#?}", tokenizer);
    tokenizer.instructions
}

fn keyword_map() -> HashMap<String, TokenType> {
    let mut map = HashMap::new();
    map.insert("A".into(), TokenType::Register("A".into()));
    map.insert("D".into(), TokenType::Register("D".into()));
    map.insert("M".into(), TokenType::Register("M".into()));
    map.insert("JGT".into(), TokenType::Jump("JGT".into()));
    map.insert("JEQ".into(), TokenType::Jump("JEQ".into()));
    map.insert("JGE".into(), TokenType::Jump("JGE".into()));
    map.insert("JLT".into(), TokenType::Jump("JLT".into()));
    map.insert("JNE".into(), TokenType::Jump("JNE".into()));
    map.insert("JLE".into(), TokenType::Jump("JLE".into()));
    map.insert("JMP".into(), TokenType::Jump("JMP".into()));
    map.insert("R0".into(), TokenType::Num(0));
    map.insert("R1".into(), TokenType::Num(1));
    map.insert("R2".into(), TokenType::Num(2));
    map.insert("R3".into(), TokenType::Num(3));
    map.insert("R4".into(), TokenType::Num(4));
    map.insert("R5".into(), TokenType::Num(5));
    map.insert("R6".into(), TokenType::Num(6));
    map.insert("R7".into(), TokenType::Num(8));
    map.insert("R8".into(), TokenType::Num(8));
    map.insert("R9".into(), TokenType::Num(9));
    map.insert("R10".into(), TokenType::Num(10));
    map.insert("R11".into(), TokenType::Num(11));
    map.insert("R12".into(), TokenType::Num(12));
    map.insert("R13".into(), TokenType::Num(13));
    map.insert("R14".into(), TokenType::Num(14));
    map.insert("R15".into(), TokenType::Num(15));
    map.insert("SP".into(), TokenType::Num(0));
    map.insert("LCL".into(), TokenType::Num(1));
    map.insert("ARG".into(), TokenType::Num(2));
    map.insert("THIS".into(), TokenType::Num(3));
    map.insert("THAT".into(), TokenType::Num(4));
    map.insert("SCREEN".into(), TokenType::Num(16384));
    map.insert("KBD".into(), TokenType::Num(24576));
    map
}

// Character Kind
#[derive(Debug, PartialEq)]
pub enum CharacterType {
    Whitespace, // ' '
    NewLine,    // ' \n'
    Alphabetic,
    Digit,
    NonAlphabetic(char),
    // Unknown(char),
}

#[derive(Debug, Clone)]
struct Tokenizer {
    p: Rc<Vec<char>>, //input
    pos: usize,
    // tokens: Vec<Token>,
    instructions: Vec<Instruction>,
    symbol: Symbol,
}

impl Tokenizer {
    fn new(context: Rc<String>) -> Self {
        Tokenizer {
            p: Rc::new(context.chars().collect()),
            pos: 0,
            // tokens: vec![],
            instructions: vec![],
            symbol: Symbol::new(),
        }
    }

    fn new_token(&self, ty: TokenType) -> Token {
        Token::new(ty)
    }

    // This does not support non-ASCII characters.
    fn get_character(&self, advance_from_pos: usize) -> Option<CharacterType> {
        self.p.get(self.pos + advance_from_pos).map(|ch| {
            if ch == &'\n' || ch == &'\r' {
                CharacterType::NewLine
            } else if ch == &' ' || ch == &'\t' {
                CharacterType::Whitespace
            } else if ch.is_alphabetic() || ch == &'_' {
                CharacterType::Alphabetic
            } else if ch.is_digit(10) {
                CharacterType::Digit
            } else {
                CharacterType::NonAlphabetic(*ch)
            }
        })
    }

    fn scan(&mut self, keywords: &HashMap<String, TokenType>) {
        let mut new_line = false;
        let mut pc = Pc::new();
        while let Some(head_char) = self.get_character(0) {
            match head_char {
                CharacterType::NewLine => {
                    self.pos += 1;
                    new_line = true;
                },
                CharacterType::Whitespace => self.pos += 1,
                CharacterType::Alphabetic => {
                    let t = self.ident(&keywords);
                    if new_line {
                        let ins = Instruction{ty: InstructionType::C, pc: pc.get(), tokens: vec![t] };
                        self.instructions.push(ins);
                        new_line = false;
                    } else {
                        self.instructions.last_mut().unwrap().tokens.push(t);
                    }
                },
                CharacterType::Digit => {
                    let t = self.number();
                    if new_line {
                        let ins = Instruction{ty: InstructionType::C, pc: pc.get(), tokens: vec![t] };
                        self.instructions.push(ins);
                        new_line = false;
                    } else {
                        self.instructions.last_mut().unwrap().tokens.push(t);
                    }
                    
                },
                CharacterType::NonAlphabetic(c) => {
                    // Single-letter symbol
                    if let Some(ty) = TokenType::new_single_letter(c) {
                        let t = self.new_token(ty);
                        self.pos += 1;
                        if t.ty == TokenType::AT {
                            let ins = Instruction{ty: InstructionType::A, pc: pc.get(), tokens: vec![t] };
                            self.instructions.push(ins);
                            new_line = false;
                        } else if new_line {
                            let ins = Instruction{ty: InstructionType::Label, pc: pc.get_without_inc(), tokens: vec![t] };
                            self.instructions.push(ins);
                            new_line = false; 
                        } else {
                            self.instructions.last_mut().unwrap().tokens.push(t);
                        }
                        // self.tokens.push(t);
                        continue;
                    }
                    panic!("Unknwon character type. '{}'", c)
                }
                // CharacterType::Unknown(_) => self.bad_position("Unknwon character type."),
            }
        }
        // self.tokens.push(self.new_token(TokenType::Eof));
        // self.tokens.clone()
    }

    fn ident(&mut self, keywords: &HashMap<String, TokenType>) -> Token{
        let mut len = 1;
        while let Some(c2) = self.p.get(self.pos + len) {
            if c2.is_alphabetic() || c2.is_ascii_digit() || c2 == &'_' {
                len += 1;
                continue;
            }
            break;
        }

        let name: String = self.p[self.pos..self.pos + len].iter().collect();
        let t;
        if let Some(keyword) = keywords.get(&name) {
            t = self.new_token(keyword.clone());
        } else {
            t = self.new_token(TokenType::Ident(name.clone()));
        }
        self.pos += len;
        t
    }

    fn number(&mut self) -> Token{
        let base = 10;
        let mut sum: i32 = 0;
        let mut len = 0;
        for c in self.p[self.pos..].iter() {
            if let Some(val) = c.to_digit(base) {
                sum = sum * base as i32 + val as i32;
                len += 1;
            } else {
                break;
            }
        }
        let t = self.new_token(TokenType::Num(sum as i32));
        self.pos += len;
        // self.tokens.push(t);
        t
    }

    // fn bad_position(&self, msg: &'static str) {
    //     panic!("{}", msg);
    // }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType, // Token type
}


impl Token {
    pub fn new(ty: TokenType) -> Self {
        Token { ty }
    }

    // pub fn is_ident(&self, s: &str) -> bool {
    //     match self.ty {
    //         TokenType::Ident(ref name) => name == s,
    //         _ => false,
    //     }
    // }
}


// Token type
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Num(i32),      // Number literal
    Ident(String), // Identifier
    Neg,           // -
    Add,           // +
    AT,             // @
    Equal,         // =
    Semicolon,     // ;
    LeftParen,     // (
    RightParen,    // )
    Register(String),
    Jump(String),
}


impl TokenType {
    fn new_single_letter(c: char) -> Option<Self> {
        use self::TokenType::*;
        match c {
            '-' => Some(Neg),
            '+' => Some(Add),
            '@' => Some(AT),
            ';' => Some(Semicolon),
            '=' => Some(Equal),
            '(' => Some(LeftParen),
            ')' => Some(RightParen),
            _ => None,
        }
    }
}