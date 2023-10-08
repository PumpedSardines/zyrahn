//! The abstract syntax tree generator for zyrahn.

use crate::*;

pub mod node;
use node::Node;

mod block;
mod expression;

pub fn gen(
    tokens: &Vec<lexer::Token>,
) -> Result<
    Vec<Node<node::block::All<Node<node::expression::All>>>>,
    error::Error<error::ParserErrorType>,
> {
    block::gen(&tokens)
}
