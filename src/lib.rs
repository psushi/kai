mod expression;
pub mod parser;
pub mod scanner;
mod token;

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::expression::*;
    use super::scanner::*;
    use super::token::*;
    #[test]
    fn it_parses_parenthesis() {
        let mut scanner = Scanner::new("()");

        let _ = scanner.scan_tokens();

        let left_paren = Token::new(TokenType::LeftParen, "(", 1);
        let right_paren = Token::new(TokenType::RightParen, ")", 1);
        let eof = Token::new(TokenType::Eof, "", 1);

        assert_eq!(vec![left_paren, right_paren, eof], scanner.tokens)
    }

    #[test]
    fn it_pretty_prints() {
        let a = Box::new(Expression::Literal(TokenType::Number(3.0)));
        let b = Box::new(Expression::Literal(TokenType::Number(6.0)));
        let exp = Expression::Binary(a, TokenType::Plus, b);
        let una = Expression::Unary(TokenType::Minus, Box::new(exp));

        println!("{}", una);
    }
    #[test]
    fn it_prints_rpn() {
        let a = Box::new(Expression::Literal(TokenType::Number(1.0)));
        let b = Box::new(Expression::Literal(TokenType::Number(2.0)));
        let exp1 = Expression::Binary(a, TokenType::Plus, b);

        let a = Box::new(Expression::Literal(TokenType::Number(4.0)));
        let b = Box::new(Expression::Literal(TokenType::Number(3.0)));
        let exp2 = Expression::Binary(a, TokenType::Minus, b);

        let exp = Expression::Binary(Box::new(exp1), TokenType::Star, Box::new(exp2));

        println!("{}", exp.print_rpn());
    }

    #[test]
    fn it_parses_ternary() {
        let mut scanner = Scanner::new("1 ? 2 : 3");

        let _ = scanner.scan_tokens();
        let exp = Parser::new(scanner.tokens).expression();

        let one = Box::new(Expression::Literal(TokenType::Number(1.0)));
        let two = Box::new(Expression::Literal(TokenType::Number(2.0)));
        let three = Box::new(Expression::Literal(TokenType::Number(3.0)));
        let ternary = Expression::Ternary(one, two, three);

        assert_eq!(ternary, exp)
    }
}
