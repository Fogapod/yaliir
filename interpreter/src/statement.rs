use crate::expression::Expr;
use crate::token::Token;

pub enum Stmt {
    Print {
        value: Expr,
    },
    Expression {
        value: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Stmt::Expression { value } => visitor.visit_expression(value),
            Stmt::Print { value } => visitor.visit_print(value),
            Stmt::Var { name, initializer } => visitor.visit_var(name, initializer),
            Stmt::Block { statements } => visitor.visit_block(statements),
        }
    }
}

pub trait Visitor<T> {
    fn visit_expression(&mut self, value: &Expr) -> T;
    fn visit_print(&mut self, value: &Expr) -> T;
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> T;
    fn visit_block(&mut self, statements: &[Stmt]) -> T;
}
