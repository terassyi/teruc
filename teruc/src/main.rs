use asm::x86::intel::constants;
use clap::Parser;
use cmd::Args;

mod cmd;

fn main() {
    let args = Args::parse();

    println!("{} {}", constants::INTEL_SYNTAX, constants::NOPREFIX);
    println!("{} main", constants::SEC_GLOBAL);
    println!("main:");
    let n: i32 = args.input.parse().unwrap();
    println!("\tmov rax, {}", n);
    println!("\tret");
}
