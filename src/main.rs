use std::fs;

use clap::Parser;

use crate::parser::tokenize;

mod parser;
mod instruction;
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
    tokenize(src);
}
