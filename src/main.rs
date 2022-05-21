use std::fs;

use clap::Parser;

use crate::{parser::tokenize, codegen::write_bit};

mod parser;
mod instruction;
mod symbols;
mod codegen;
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    input: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let input = &cli.input.unwrap();
    let src = fs::read_to_string(input).expect("src not existed.");
    println!("src:\n{}", src);
    let ins = tokenize(src);
    write_bit(ins);
}
