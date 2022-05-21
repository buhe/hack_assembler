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

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let input = &cli.input.unwrap();
    let src = fs::read_to_string(input).expect("src not existed.");
    println!("src:\n{}", src);
    let ins = tokenize(src);
    let bit = write_bit(ins);

    let target = input.replace(".asm", "");
    let hack_file = format!("{}.hack", target);
    fs::write(&hack_file, &bit)?;
    Ok(())
}
