use error::Error;
use parser::ast::{Node, NodeKind};

mod error;
pub fn generate(node: &Node) -> Result<(), Error> {
    match node.kind {
        NodeKind::Num(n) => {
            println!("\tpush {n}");
            return Ok(());
        }
        NodeKind::LocalVar(_, _) => {
            generate_local_val(node)?;
            println!("\tpop rax");
            println!("\tmov rax, [rax]");
            println!("\tpush rax");
            return Ok(());
        }
        NodeKind::Assignment => {
            if let Some(lhs) = &node.lhs {
                generate_local_val(lhs)?;
            } else {
                return Err(Error::InvalidNode);
            }
            if let Some(rhs) = &node.rhs {
                generate(rhs)?;
            } else {
                return Err(Error::InvalidNode);
            }
            println!("\tpop rdi");
            println!("\tpop rax");
            println!("\tmov [rax], rdi");
            println!("\tpush rdi");
            return Ok(());
        }
        NodeKind::Return => {
            if let Some(lhs) = &node.lhs {
                generate(lhs)?;
            } else {
                return Err(Error::InvalidNode);
            }
            println!("\tpop rax");
            println!("\tmov rsp, rbp");
            println!("\tpop rbp");
            println!("\tret");
            return Ok(());
        }
        _ => {}
    }

    if let Some(lhs) = &node.lhs {
        generate(lhs)?;
    } else {
        return Err(Error::InvalidNode);
    }
    if let Some(rhs) = &node.rhs {
        generate(rhs)?;
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
        NodeKind::Equal => {
            println!("\tcmp rax, rdi");
            println!("\tsete al");
            println!("\tmovzb rax, al");
        }
        NodeKind::NotEqual => {
            println!("\tcmp rax, rdi");
            println!("\tsetne al");
            println!("\tmovzb rax, al");
        }
        NodeKind::LessThan => {
            println!("\tcmp rax, rdi");
            println!("\tsetl al");
            println!("\tmovzb rax, al");
        }
        NodeKind::LessThanOrEqual => {
            println!("\tcmp rax, rdi");
            println!("\tsetle al");
            println!("\tmovzb rax, al");
        }
        _ => return Err(Error::InvalidNode),
    }

    println!("\tpush rax");

    Ok(())
}

fn generate_local_val(node: &Node) -> Result<(), Error> {
    if let NodeKind::LocalVar(_s, offset) = &node.kind {
        println!("\tmov rax, rbp");
        println!("\tsub rax, {offset}");
        println!("\tpush rax");
        Ok(())
    } else {
        Err(Error::LeftValueMustBeIdentifier)
    }
}
