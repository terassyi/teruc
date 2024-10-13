use asm::x86::intel::constants;
use clap::Parser;
use cmd::Args;
use tokenizer::{Token, Tokenizer};

mod cmd;

fn main() {
    let args = Args::parse();

    pre_process();

    let tokenizer = Tokenizer::default();
    let mut tokens = tokenizer.process(args.input).unwrap();
    let mut token_iter = tokens.iter_mut();

    if let Some(Token::Num(n)) = token_iter.next() {
        println!("  mov rax, {}", n);
    } else {
        panic!("First token should be number");
    }

    while let Some(t) = token_iter.next() {
        if let Token::Reserved(p) = t {
            if '+'.eq(p) {
                if let Some(Token::Num(n)) = token_iter.next() {
                    println!("  add rax, {}", n)
                }
            } else if '-'.eq(p) {
                if let Some(Token::Num(n)) = token_iter.next() {
                    println!("  sub rax, {}", n)
                }
            }
        }
    }
    println!("  ret")
}

fn pre_process() {
    println!("{} {}", constants::INTEL_SYNTAX, constants::NOPREFIX);
    println!("{} main", constants::SEC_GLOBAL);
    println!("main:");
}
