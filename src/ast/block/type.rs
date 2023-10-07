use super::*;
use cl_ln::ClLn;

enum TypeToken {
    Token(lexer::Token),
    Type(block::Type),
}

impl cl_ln::ClLn for TypeToken {
    fn cl_start(&self) -> usize {
        match self {
            TypeToken::Token(t) => t.cl_start(),
            TypeToken::Type(t) => t.cl_start(),
        }
    }

    fn cl_end(&self) -> usize {
        match self {
            TypeToken::Token(t) => t.cl_end(),
            TypeToken::Type(t) => t.cl_end(),
        }
    }

    fn ln_start(&self) -> usize {
        match self {
            TypeToken::Token(t) => t.ln_start(),
            TypeToken::Type(t) => t.ln_start(),
        }
    }

    fn ln_end(&self) -> usize {
        match self {
            TypeToken::Token(t) => t.ln_end(),
            TypeToken::Type(t) => t.ln_end(),
        }
    }
}

pub fn gen(tokens: &[lexer::Token]) -> Result<block::Type, error::Error<error::AstErrorType>> {
    if tokens.len() == 0 {
        panic!("parse_type called with no tokens");
    }

    let tokens: Vec<TypeToken> = tokens
        .into_iter()
        .map(|t| TypeToken::Token(t.clone()))
        .collect();

    parse_type(&tokens)
}

fn parse_type(tokens: &[TypeToken]) -> Result<block::Type, error::Error<error::AstErrorType>> {
    if tokens.len() == 0 {
        panic!("parse_type called with no tokens");
    }

    if tokens.len() != 1 {
        return Err(error::Error::from_cl_ln(
            error::AstErrorType::FeatureNotImplemented("Array and struct types".to_string()),
            &tokens[0].cl_ln(),
        ));
    }

    match &tokens[0] {
        TypeToken::Token(t) => match t.token_type {
            lexer::TokenType::Float => Ok(block::Type::Float { cl_ln: t.cl_ln() }),
            lexer::TokenType::Integer => Ok(block::Type::Integer { cl_ln: t.cl_ln() }),
            lexer::TokenType::String => Ok(block::Type::String { cl_ln: t.cl_ln() }),
            lexer::TokenType::Boolean => Ok(block::Type::Boolean { cl_ln: t.cl_ln() }),
            _ => Err(error::Error::from_cl_ln(
                error::AstErrorType::UnexpectedToken(t.token_type.clone()),
                t,
            )),
        },
        TypeToken::Type(_) => unreachable!(),
    }
}
