use std::rc::Rc;

#[derive(Debug, Clone)]
struct Tokenizer {
    p: Rc<Vec<char>>, //input
    pos: usize,
    tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType, // Token type
}


// Token type
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Num(i32),      // Number literal
    Ident(String), // Identifier
    Semicolon,     // ;
    Eof,
}