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

impl Program {
    // fn token(&self) -> &str {
    //     match self.statements.is_empty() {
    //         true => "",
    //         false => self.statements.first().unwrap().token(),
    //     }
    // }
}

pub enum Statement {
    Var(VarStatement),
}

pub struct VarStatement {
    // token: Token, // Token.Var 
    // name: Identifier,
    // value: Box<dyn Expression>,
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
//     token: Token, // Token.Ident
//     // value: String,
// }
//
// impl Expression for Identifier {
//     fn expression_node(&self) {
//         todo!()
//     }
// }
//
// impl Node for Identifier {
//     fn token(&self) -> Token {
//         "x".to_string()
//         // return self.token.to_string().as_str();
//     }
// }
