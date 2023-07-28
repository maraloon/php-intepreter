pub trait Node {
    fn string(&self) -> String;
}

pub trait StatementNode: Node {
    fn statement_node(&self);
}

// trait ExpressionNode: Node {
//     fn expression_node(&self);
// }

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn string(&self) -> String {
        let mut out = "".to_owned();
        for statement in &self.statements {
            out.push_str(&statement.string());
        }
        out
    }
}

pub enum Statement {
    Var(VarStatement),
}
impl Statement {
    fn string(&self) -> String {
        match self {
            Statement::Var(s) => s.string(),
        }
    }
}

pub struct VarStatement {
    pub name: String,
    pub value: String,
}

impl StatementNode for VarStatement {
    fn statement_node(&self) {
        todo!()
    }
}

impl Node for VarStatement {
    fn string(&self) -> String {
        return "$".to_owned() + &self.name + " = " + &self.value + ";";
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, Program, Statement, VarStatement};

    #[test]
    fn string() {
        let program = Program {
            statements: vec![
                Statement::Var(VarStatement {
                    name: "x".to_string(),
                    value: "5".to_string(),
                }),
                Statement::Var(VarStatement {
                    name: "y".to_string(),
                    value: "7".to_string(),
                }),
            ],
        };

        if program.string() != "$x = 5;$y = 7;".to_owned() {
            panic!(
                "program.string() wrong. expexted={}, got={}",
                "$x = 5;$y = 7;",
                program.string()
            );
        }
    }
}
