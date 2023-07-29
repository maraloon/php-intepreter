pub trait Node {
    fn string(&self) -> String;
}

pub trait StatementNode: Node {}
trait ExpressionNode: Node {}

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
    Return(ReturnStatement),
}
impl Statement {
    fn string(&self) -> String {
        match Option::Some(self) {
            Some(statement) => statement.string(),
            _ => panic!("fail string()"),
        }
    }
}
// todo another statement. To see the structure and maybe not use enum Statement
// todo Expressions for value

pub struct VarStatement {
    pub name: String,
    pub value: String,
}

impl Node for VarStatement {
    fn string(&self) -> String {
        return "$".to_owned() + &self.name + " = " + &self.value + ";";
    }
}

pub struct ReturnStatement {
    pub return_value: String,
}

impl Node for ReturnStatement {
    fn string(&self) -> String {
        return "return ".to_owned() + &self.return_value;
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
