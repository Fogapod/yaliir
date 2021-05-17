use crate::object::Object;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        object: Object,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        token: Token,
        value: Box<Expr>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Expr::Assign { name, value } => visitor.visit_assign(name, value),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call(callee, paren, arguments),
            Expr::Get { object, name } => visitor.visit_get(object, name),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { object } => visitor.visit_literal(object),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical(left, operator, right),
            Expr::Set {
                object,
                token,
                value,
            } => visitor.visit_set(object, token, value),
            Expr::Super { keyword, method } => visitor.visit_super(keyword, method),
            Expr::This { keyword } => visitor.visit_this(keyword),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Variable { name } => visitor.visit_variable(name),
        }
    }
}
pub trait Visitor<R> {
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> R;
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_call(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> R;
    fn visit_get(&mut self, object: &Expr, name: &Token) -> R;
    fn visit_grouping(&mut self, expression: &Expr) -> R;
    fn visit_literal(&mut self, object: &Object) -> R;
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_set(&mut self, object: &Expr, token: &Token, value: &Expr) -> R;
    fn visit_super(&mut self, keyword: &Token, method: &Token) -> R;
    fn visit_this(&mut self, keyword: &Token) -> R;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> R;
    fn visit_variable(&mut self, name: &Token) -> R;
}
