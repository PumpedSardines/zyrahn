use super::*;

pub fn gen(tokens: &[lexer::Token]) -> Result<block::All, error::Error<error::AstErrorType>> {
    if tokens.len() == 0 {
        panic!("var_dec called with no tokens");
    }

    if tokens.len() < 6 {
        return Err(error::Error::from_cl_ln(
            error::AstErrorType::StatementEndEarly,
            &tokens[0],
        ));
    }

    let variable_name = match &tokens[1].token_type {
        lexer::TokenType::Identifier(s) => s,
        _ => {
            return Err(error::Error::from_cl_ln(
                error::AstErrorType::MissingIdentifier,
                &tokens[0],
            ));
        }
    };

    if tokens[2].token_type != lexer::TokenType::Colon {
        return Err(error::Error::from_cl_ln(
            error::AstErrorType::UnexpectedTokenExpected(
                tokens[2].token_type.clone(),
                lexer::TokenType::Colon,
            ),
            &tokens[2],
        ));
    }

    for i in 3..tokens.len() {
        if tokens[i].token_type == lexer::TokenType::Assign {
            let ty = &tokens[3..i];
            let rest_tokens = &tokens[i + 1..];

            if ty.len() == 0 {
                return Err(error::Error::from_cl_ln(
                    error::AstErrorType::UnexpectedToken(lexer::TokenType::Assign),
                    &tokens[3],
                ));
            }

            let ty = r#type::gen(ty)?;
            let exp = expression::gen(rest_tokens)?;

            return Ok(block::All::VariableDeclaration {
                identifier: variable_name.to_string(),
                r#type: ty,
                value: exp,
                cl_ln: cl_ln::combine(tokens),
            });
        }
    }

    todo!()
}
