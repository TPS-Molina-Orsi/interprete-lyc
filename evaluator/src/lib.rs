use parser::Expression;
use parser::LiteralValue;
use scanner::TokenType;
use scanner::Token;

#[derive(Debug, Clone)]
pub struct Evaluator {
    expr: Expression
}

impl Evaluator {
    pub fn new(expr: Expression) -> Self {
        return Evaluator { expr: expr }
    }

    pub fn evaluate(&mut self) -> f64 {
        self._evaluate(self.expr.clone())
    }

    fn _evaluate(&mut self, expr: Expression) -> f64 {
        match expr {
            Expression::Literal { value } => match value {
                LiteralValue::Number(n) => n,
                LiteralValue::Boolean(b) => {
                    if b { 1.0 } else { 0.0 }
                }
                LiteralValue::String(_) | LiteralValue::Nil => {
                    panic!("No se puede evaluar String o Nil como número");
                }
            },
            Expression::Procedure{operator, operands} =>  {
                let evaluated: Vec<f64> = operands.iter().map(|op| self._evaluate(op.clone())).collect();
                    match operator.token_type {
                        TokenType::Plus => evaluated.iter().sum(),
                        TokenType::Minus => {
                            let first = evaluated[0];
                            first - evaluated[1..].iter().sum::<f64>()
                        }
                        TokenType::Star => evaluated.iter().product(),
                        TokenType::ForwardSlash => {
                            let first = evaluated[0];
                            evaluated[1..].iter().fold(first, |acc, x| acc / x)
                        }, // estoy ignorando el resto de casos
                        _default =>
                            0.0
                }
            },
            Expression::Unary { operand, expr} => {
                0.0
            }
        }
    }

}