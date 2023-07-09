use crate::lexer::lexer::{Token, Value};

pub trait Node {
    fn token(&self) -> Token;
}

pub trait StatementNode: Node {
    fn statement_node(&self);
}

trait ExpressionNode: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Var(VarStatement),
}

pub struct VarStatement {
    pub token: Token,
    pub name: String,
    pub value: String,
}

impl StatementNode for VarStatement {
    fn statement_node(&self) {
        todo!()
    }
}

impl Node for VarStatement {
    fn token(&self) -> Token {
        return Token::Var(Value::Empty); 
    }
}

// pub struct Identifier {
//     pub token: Token,
//     // pub value: String,
//     pub value: Value,
// }
//
// impl ExpressionNode for Identifier {
//     fn expression_node(&self) {
//         todo!()
//     }
// }
//
// impl Node for Identifier {
//     fn token(&self) -> Token {
//         return Token::Ident(Value::Empty); 
//     }
// }
