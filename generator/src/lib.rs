use error::Error;
use parser::ast::{Node, NodeKind};

mod error;

#[derive(Debug, Default)]
pub struct Generator {
    end_labels: u32,
    else_labels: u32,
    begin_labels: u32,
}

impl Generator {
    pub fn generate(&mut self, node: &Node) -> Result<(), Error> {
        eprintln!("generating node => {:?}\n", node);
        match &node.kind {
            NodeKind::Num(n) => {
                println!("\tpush {n}");
                return Ok(());
            }
            NodeKind::LocalVar(_, _) => {
                self.generate_local_val(node)?;
                println!("\tpop rax");
                println!("\tmov rax, [rax]");
                println!("\tpush rax");
                return Ok(());
            }
            NodeKind::Assignment => {
                if let Some(lhs) = &node.lhs {
                    self.generate_local_val(lhs)?;
                } else {
                    return Err(Error::InvalidNode);
                }
                if let Some(rhs) = &node.rhs {
                    self.generate(rhs)?;
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
                    self.generate(lhs)?;
                } else {
                    return Err(Error::InvalidNode);
                }
                println!("\tpop rax");
                println!("\tmov rsp, rbp");
                println!("\tpop rbp");
                println!("\tret");
                return Ok(());
            }
            NodeKind::If => {
                if let Some(lhs) = &node.lhs {
                    self.generate(lhs)?;
                } else {
                    return Err(Error::InvalidNode);
                }
                println!("\tpop rax");
                println!("\tcmp rax, 0");
                if let Some(rhs) = &node.rhs {
                    if rhs.kind.eq(&NodeKind::Else) {
                        println!("\tje .Lelse{}", self.else_labels);
                        // gen some code
                        if let Some(lhs) = &rhs.lhs {
                            self.generate(lhs)?;
                            println!("\tjmp .Lend{}", self.end_labels);
                        }
                        if let Some(rhs) = &rhs.rhs {
                            println!(".Lelse{}:", self.else_labels);
                            self.else_labels += 1;
                            self.generate(rhs)?;
                        }
                        // TODO: handle duplicate .LendX when handling "else if"
                        println!(".Lend{}:", self.end_labels);
                        self.end_labels += 1;
                    } else {
                        println!("\tje .Lend{}", self.end_labels);
                        self.generate(rhs)?;
                        println!(".Lend{}:", self.end_labels);
                        self.end_labels += 1;
                    }
                } else {
                    return Err(Error::InvalidNode);
                }
                return Ok(());
            }
            NodeKind::While => {
                println!(".Lbegin{}:", self.begin_labels);
                if let Some(lhs) = &node.lhs {
                    self.generate(lhs)?;
                } else {
                    return Err(Error::InvalidNode);
                }
                println!("\tpop rax");
                println!("\tcmp rax, 0");
                println!("\tje .Lend{}", self.end_labels);
                if let Some(rhs) = &node.rhs {
                    self.generate(rhs)?;
                } else {
                    return Err(Error::InvalidNode);
                }
                println!("\tjmp .Lbegin{}", self.begin_labels);
                self.begin_labels += 1;
                println!(".Lend{}:", self.end_labels);
                self.end_labels += 1;
                return Ok(());
            }
            NodeKind::For => {
                if let Some(lhs) = &node.lhs {
                    self.generate(lhs)?;
                } else {
                    return Err(Error::InvalidNode);
                }
                println!(".Lbegin{}", self.begin_labels);
                if let Some(rhs) = &node.rhs {
                    if rhs.kind.ne(&NodeKind::If) {
                        return Err(Error::InvalidNode);
                    }
                    if let Some(lhs) = &rhs.lhs {
                        self.generate(lhs)?;
                    } else {
                        return Err(Error::InvalidNode);
                    }
                    println!("\tpop rax");
                    println!("\tcmp rax, 0");
                    println!("\tje .Lend{}", self.end_labels);
                    if let Some(rhs) = &rhs.rhs {
                        self.generate(rhs)?;
                    }
                    println!("\tjmp .Lbegin{}", self.begin_labels);
                    self.begin_labels += 1;
                }
                println!(".Lend{}", self.end_labels);
                self.end_labels += 1;
                return Ok(());
            }
            NodeKind::Block(nodes) => {
                for node in nodes.iter() {
                    self.generate(node)?;
                    println!("\tpop rax");
                }
                return Ok(());
            }
            _ => {}
        }

        if let Some(lhs) = &node.lhs {
            self.generate(lhs)?;
        } else {
            return Err(Error::InvalidNode);
        }
        if let Some(rhs) = &node.rhs {
            self.generate(rhs)?;
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

    fn generate_local_val(&self, node: &Node) -> Result<(), Error> {
        eprintln!("generate local ver => {:?}\n", node);
        if let NodeKind::LocalVar(_s, offset) = &node.kind {
            println!("\tmov rax, rbp");
            println!("\tsub rax, {offset}");
            println!("\tpush rax");
            Ok(())
        } else {
            Err(Error::LeftValueMustBeIdentifier)
        }
    }
}
