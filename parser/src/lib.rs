use scanner::TokenType;
use scanner::Token;
use log;


#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left_expr: Box<Expression>,
        operator: Token,
        right_expr: Box<Expression>
    }, 
    Unary {
        operand: TokenType,
        expr: Box<Expression>
    },
    Literal {
        value: LiteralValue
    }
}


impl Expression {
    pub fn new_binary(left_expr: Expression, operator: Token, right_expr: Expression) -> Self {
        return Expression::Binary { left_expr: Box::new(left_expr), operator: operator, right_expr: Box::new(right_expr) }
    }

    pub fn new_literal(value: LiteralValue) -> Self {
        Expression::Literal { value }
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    index_actual_token: usize,
    count_open_parenthesis: i32,
    count_close_parenthesis: i32, 
    result_expressions: Vec<Expression>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { 
            tokens: tokens,
            index_actual_token: 0,
            count_open_parenthesis: 0,
            count_close_parenthesis: 0,
            result_expressions: Vec::new()
          }
    }

    pub fn parse(&mut self) -> Expression {
        log::trace!("{:#?}", self.peek());
        if self.peek().token_type == TokenType::OpenParens {
            self.count_open_parenthesis += 1;
            self.consume(); // Consume the parens
            if !self.match_token(vec![
                    TokenType::Equal, 
                    TokenType::Great, 
                    TokenType::GreatEqual, 
                    TokenType::Less, 
                    TokenType::LessEqual,
                    TokenType::Star, 
                    TokenType::Plus, 
                    TokenType::Minus, 
                    TokenType::ForwardSlash
                ]) {
                    panic!("Wrong operator");
            }
            let operator = self.previous().clone();
            return Expression::new_binary(self.parse(), operator, self.parse())
        } else if self.peek().token_type == TokenType::CloseParens {
            self.count_close_parenthesis += 1;
            self.consume();
        }

        if let TokenType::Number { literal } = &self.peek().token_type {
            let lit = literal.clone();
            self.consume();
            let number: f64 = lit.parse::<f64>()
                .expect("Invalid number literal");
            println!("en number: ");
            log::trace!("{:#?}", self.peek());
            return Expression::new_literal(LiteralValue::Number(number));
        } else if let TokenType::String { literal } = &self.peek().token_type {
            let lit = literal.clone();
            self.consume();
            return Expression::new_literal(LiteralValue::String(lit));
        } else {
            return Expression::new_literal(LiteralValue::Nil)
        }
    }



    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.peek().token_type == token_type {
                self.consume();
                return true;
            }
        }
        return false;
    }

    fn is_at_end(&self) -> bool {
        return self.index_actual_token == self.tokens.len();
    }

    fn consume(&mut self) -> &Token {
        if self.is_at_end() {
            panic!("There are no more tokens to consume!");
        }
        self.index_actual_token += 1;
        return &self.tokens[self.index_actual_token - 1];
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.index_actual_token];
    }

    fn previous(&self) -> &Token {
        if self.index_actual_token == 0 {
            panic!("There is no previous token");
        } else {
            return &self.tokens[self.index_actual_token - 1];
        }
    }
}

