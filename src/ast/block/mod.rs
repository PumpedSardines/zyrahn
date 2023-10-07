use super::*;
use cl_ln::ClLn;
use node::block;

mod r#type;
mod var_dec;

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
    if tokens.len() == 0 {
        return Ok(vec![]);
    }

    match &tokens[0].token_type {
        lexer::TokenType::Var => {
            for i in 0..tokens.len() {
                if tokens[i].token_type == lexer::TokenType::Semicolon {
                    let var_dec_tokens = &tokens[0..i];
                    let rest_tokens = &tokens[i + 1..];

                    let var_dec = var_dec::gen(var_dec_tokens)?;
                    let mut rest = gen(rest_tokens)?;
                    rest.insert(0, var_dec);

                    return Ok(rest);
                }
            }

            return Err(error::Error::from_cl_ln(
                error::AstErrorType::MissingSemicolon,
                &tokens[0],
            ));
        }
        _ => {
            for i in 0..tokens.len() {
                if tokens[i].token_type == lexer::TokenType::Semicolon {
                    let expression_tokens = &tokens[0..i];
                    let rest_tokens = &tokens[i + 1..];

                    let expression = expression::gen(expression_tokens)?;
                    let mut rest = gen(rest_tokens)?;
                    let cl_ln = expression.cl_ln();

                    rest.insert(
                        0,
                        block::All::Expression {
                            value: expression,
                            cl_ln,
                        },
                    );

                    return Ok(rest);
                }
            }

            return Err(error::Error::from_cl_ln(
                error::AstErrorType::MissingSemicolon,
                &tokens[0],
            ));
        }
    }
}
