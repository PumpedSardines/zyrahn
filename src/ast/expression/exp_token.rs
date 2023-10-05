//! An internal representation of a token when evaluating an expression
//! Used to combine already evaluated expressions with tokens that are not yet evaluated

use crate::*;

#[derive(Debug, Clone)]
pub(super) enum ExpressionToken {
    Token(lexer::Token),
    Expression(ast::node::expression::All),
}

impl cl_ln::ClLn for ExpressionToken {
    fn cl_start(&self) -> usize {
        match self {
            ExpressionToken::Token(token) => token.cl_start(),
            ExpressionToken::Expression(expression) => expression.cl_start(),
        }
    }

    fn cl_end(&self) -> usize {
        match self {
            ExpressionToken::Token(token) => token.cl_end(),
            ExpressionToken::Expression(expression) => expression.cl_end(),
        }
    }

    fn ln_start(&self) -> usize {
        match self {
            ExpressionToken::Token(token) => token.ln_start(),
            ExpressionToken::Expression(expression) => expression.ln_start(),
        }
    }

    fn ln_end(&self) -> usize {
        match self {
            ExpressionToken::Token(token) => token.ln_end(),
            ExpressionToken::Expression(expression) => expression.ln_end(),
        }
    }
}
