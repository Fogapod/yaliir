use crate::expression::Expr;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        value: Expr,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        value: Expr,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Stmt::Block { statements } => visitor.visit_block(statements),
            Stmt::Expression { value } => visitor.visit_expression(value),
            Stmt::Function { name, params, body } => visitor.visit_function(name, params, body),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if(condition, then_branch, else_branch),
            Stmt::Print { value } => visitor.visit_print(value),
            Stmt::Return { keyword, value } => visitor.visit_return(keyword, value),
            Stmt::Var { name, initializer } => visitor.visit_var(name, initializer),
            Stmt::While { condition, body } => visitor.visit_while(condition, body),
        }
    }
}

pub trait Visitor<T> {
    fn visit_block(&mut self, statements: &[Stmt]) -> T;
    fn visit_expression(&mut self, value: &Expr) -> T;
    fn visit_function(&mut self, name: &Token, params: &[Token], body: &[Stmt]) -> T;
    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> T;
    fn visit_print(&mut self, value: &Expr) -> T;
    fn visit_return(&mut self, keyword: &Token, value: &Option<Expr>) -> T;
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> T;
    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> T;
}
