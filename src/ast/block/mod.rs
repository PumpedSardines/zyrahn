use super::*;
use node::block;

/// Generates an abstract syntax tree from a list of tokens for the block syntax.
///
/// # Examples
/// ```
/// var a = 3;
///
/// if a == 3 {
///
/// }
/// ```
pub fn gen(tokens: &[lexer::Token]) -> Result<Vec<block::All>, error::Error<error::AstErrorType>> {

}
