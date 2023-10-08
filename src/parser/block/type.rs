use super::*;
use cl_ln::ClLn;

enum TypeToken {
    Token(lexer::Token),
    Type(common::Type),
}

pub fn gen(tokens: &[lexer::Token]) -> Result<common::Type, error::Error<error::ParserErrorType>> {
    if tokens.len() == 0 {
        panic!("parse_type called with no tokens");
    }

    if tokens.len() != 1 {
        return Err(error::Error::from_cl_ln(
            error::ParserErrorType::FeatureNotImplemented("Array and struct types".to_string()),
            &tokens[0].cl_ln(),
        ));
    }

    match tokens[0].token_type {
        lexer::TokenType::Float => Ok(common::Type::Float),
        lexer::TokenType::Integer => Ok(common::Type::Integer),
        lexer::TokenType::String => Ok(common::Type::String),
        lexer::TokenType::Boolean => Ok(common::Type::Boolean),
        _ => Err(error::Error::from_cl_ln(
            error::ParserErrorType::UnexpectedToken(tokens[0].token_type.clone()),
            &tokens[0],
        )),
    }
}
