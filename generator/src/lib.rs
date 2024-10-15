use error::Error;
use parser::ast::{Node, NodeKind};

mod error;
pub fn generate(node: Node) -> Result<(), Error> {
    if let NodeKind::Num(n) = node.kind {
        println!("\tpush {n}");
        return Ok(());
    }

    if let Some(lhs) = node.lhs {
        generate(*lhs)?;
    } else {
        return Err(Error::InvalidNode);
    }
    if let Some(rhs) = node.rhs {
        generate(*rhs)?;
    } else {
        return Err(Error::InvalidNode);
    }

    println!("\tpop rdi");
    println!("\tpop rax");

    match node.kind {
        NodeKind::Add => println!("\tadd rax, rdi"),
        NodeKind::Sub => println!("\tsub rax, rdi"),
        NodeKind::Mul => println!("\timul rax, rdi"),
        NodeKind::Div => {
            println!("\tcqo");
            println!("\tidiv rax, rdi");
        }
        _ => return Err(Error::InvalidNode),
    }

    println!("\tpush rax");

    Ok(())
}
