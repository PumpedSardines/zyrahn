//! The abstract syntax tree generator for zyrahn.

use crate::*;

pub mod node;

mod expression;

pub fn gen(
    tokens: &Vec<lexer::Token>,
) -> Result<node::expression::All, error::Error<error::AstErrorType>> {
    expression::gen(&tokens)
}
