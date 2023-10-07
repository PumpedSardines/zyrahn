//! The abstract syntax tree generator for zyrahn.

use crate::*;

pub mod node;

mod block;
mod expression;

pub fn gen(
    tokens: &Vec<lexer::Token>,
) -> Result<Vec<node::block::All>, error::Error<error::AstErrorType>> {
    block::gen(&tokens)
}
