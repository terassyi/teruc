use asm::x86::intel::constants;
use clap::Parser;
use cmd::Args;
use parser::parser;
use tokenizer::Tokenizer;

mod cmd;

fn main() {
    let args = Args::parse();

    pre_process();

    let tokenizer = Tokenizer::default();
    let tokens = tokenizer.process(args.input).unwrap();

    let mut parser = parser::Parser::new(tokens);
    let node = parser.parse().unwrap();

    generator::generate(node).unwrap();

    println!("\tpop rax");
    println!("\tret")
}

fn pre_process() {
    println!("{} {}", constants::INTEL_SYNTAX, constants::NOPREFIX);
    println!("{} main", constants::SEC_GLOBAL);
    println!("main:");
}
