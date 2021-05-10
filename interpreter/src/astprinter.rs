use crate::expression::{Expr, Visitor};
use crate::object::Object;
use crate::token::Token;

pub struct AstPrinter {}

impl AstPrinter {
    #[allow(dead_code)]
    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();

        result.push('(');
        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self));
        }
        result.push(')');

        result
    }
}

impl Visitor<String> for AstPrinter {
    #[allow(unused_variables)]
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> String {
        todo!();
    }
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }
    #[allow(unused_variables)]
    fn visit_call(&mut self, callee: &Token, paren: &Expr, arguments: &[Expr]) -> String {
        todo!();
    }
    #[allow(unused_variables)]
    fn visit_get(&mut self, object: &Expr, name: &Token) -> String {
        todo!();
    }
    fn visit_grouping(&mut self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }
    fn visit_literal(&mut self, object: &Object) -> String {
        object.to_string()
    }
    #[allow(unused_variables)]
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        todo!();
    }
    #[allow(unused_variables)]
    fn visit_set(&mut self, object: &Expr, token: &Token, value: &Expr) -> String {
        todo!();
    }
    #[allow(unused_variables)]
    fn visit_super(&mut self, keyword: &Token, method: &Token) -> String {
        todo!();
    }
    #[allow(unused_variables)]
    fn visit_this(&mut self, keyword: &Token) -> String {
        todo!();
    }
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }
    #[allow(unused_variables)]
    fn visit_variable(&mut self, name: &Token) -> String {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::AstPrinter;

    use crate::expression::Expr;
    use crate::object::Object;
    use crate::token::{Token, TokenType};

    #[test]
    fn main() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token {
                    lexeme: "-".to_string(),
                    line: 1,
                    token_type: TokenType::Minus,
                },
                right: Box::new(Expr::Literal {
                    object: Object::Number(123.0),
                }),
            }),
            operator: Token {
                lexeme: "*".to_string(),
                line: 1,
                token_type: TokenType::Star,
            },
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    object: Object::Number(45.67),
                }),
            }),
        };

        assert_eq!(AstPrinter {}.print(expression), "(* (- 123) (group 45.67))");
    }
}
