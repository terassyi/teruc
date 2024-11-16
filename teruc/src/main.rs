use asm::x86::intel::constants;
use clap::Parser;
use cmd::Args;
use generator::Generator;
use parser::parser;
use tokenizer::Tokenizer;

mod cmd;

fn main() {
    let args = Args::parse();

    let tokenizer = Tokenizer::default();
    let tokens = tokenizer.process(args.input).unwrap();

    let mut parser = parser::Parser::new(tokens);
    parser.parse().unwrap();

    pre_process();

    alloc_local_area();

    let mut generator = Generator::default();
    for node in parser.nodes.iter() {
        generator.generate(node).unwrap();
        println!("\tpop rax");
    }

    post_process();
}

fn pre_process() {
    println!("{} {}", constants::INTEL_SYNTAX, constants::NOPREFIX);
    println!("{} main", constants::SEC_GLOBAL);
    println!("main:");
}

fn post_process() {
    println!("\tmov rsp, rbp");
    println!("\tpop rbp");
    println!("\tret")
}

fn alloc_local_area() {
    println!("\tpush rbp");
    println!("\tmov rbp, rsp");
    println!("\tsub rsp, 208"); // 8 * 26
}
