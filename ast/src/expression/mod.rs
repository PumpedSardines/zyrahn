use crate::node;
pub mod error;
mod exp;
mod single_data_unit;
// First pass do all the evaluation stuff

/// Internal representation of an expression
#[derive(Debug, Clone)]
enum ExpressionToken {
    Token(lexer::Token),
    Expression(node::expression::All),
}

fn cl_ln_from_expression_token(token: &ExpressionToken) -> node::ClLn {
    match token {
        ExpressionToken::Token(token) => node::cl_ln_from_token(token),
        ExpressionToken::Expression(expression) => expression.cl_ln(),
    }
}

fn cl_ln_from_many_expression_tokens(tokens: &[ExpressionToken]) -> node::ClLn {
    node::cl_ln_from_many_cl_ln(
        &tokens[..]
            .into_iter()
            .map(cl_ln_from_expression_token)
            .collect::<Vec<_>>(),
    )
}

/// The expression parser. An expression is anything that can be evaluated to a single value.
///
/// # Examples:
/// ```
/// 3 + 4;
/// 6 + 8 * 9 - 10;
/// (my_variable[6 * -(3 + 4)].property + 3) * 4;
/// ```
pub fn gen(tokens: &[lexer::Token]) -> Result<node::expression::All, error::Error> {
    if tokens.len() == 0 {
        panic!("Cannot parse empty expression");
    }

    // Since `exp::gen` expects that all parentheses are already calculated, this function will
    // first parse all parentheses and then send the tokens to `exp::gen`

    // How do we se the difference between a function call and a parenthesized expression?
    // - If the first token is an identifier, it's a function call
    // - Otherwise it's a parenthesized expression

    // This means that it's not possible to call a function from anything else than an identifier
    // Example:
    // (3 + 4)() // This is not possible
    // my_function() // This is possible
    // (my_function)() // This is possible
    // (my_function()) // This is not possible

    // Pretty ugly code that will parse all parentheses
    let mut ret_tokens: Vec<ExpressionToken> = vec![];
    let mut peek_tokens = tokens.iter().enumerate().peekable();
    let mut p_count = 0;
    let mut p_start: Option<usize> = None;
    while let Some((i, t)) = peek_tokens.next() {
        use lexer::TokenType::*;

        if p_start.is_some() {
            if t.token_type == ParenOpen {
                p_count += 1;
            } else if t.token_type == ParenClose {
                p_count -= 1;

                if p_count < 0 {
                    return Err(error::Error::from_token(
                        error::ErrorType::UnexpectedCloseParen,
                        t,
                    ));
                }

                if p_count == 0 {
                    let p_end = i;

                    let p_tokens = &tokens[p_start.unwrap() + 1..p_end];

                    if p_tokens.len() == 0 {
                        return Err(error::Error::from_many_tokens(
                            error::ErrorType::EmptyExpression,
                            &tokens[p_start.unwrap()..=p_end],
                        ));
                    }

                    let p_tokens = gen(p_tokens)?;

                    p_start = None;

                    ret_tokens.push(ExpressionToken::Expression(p_tokens));
                }
            }
            continue;
        }

        let can_begin_match = match &t.token_type {
            Identifier(_) => false,
            _ => true,
        };

        if i == 0 && t.token_type == ParenOpen {
            p_count += 1;
            p_start = Some(i);
            continue;
        } else if can_begin_match {
            if let Some((_, nt)) = peek_tokens.peek() {
                if nt.token_type == ParenOpen {
                    p_count += 1;
                    p_start = Some(i + 1);
                    // Hack to skip the next token
                    peek_tokens.nth(0);
                }
            }
        }

        ret_tokens.push(ExpressionToken::Token(t.clone()));
    }

    if p_count > 0 {
        return Err(error::Error::from_token(
            error::ErrorType::UnclosedExpression,
            &tokens[p_start.unwrap()],
        ));
    }

    exp::gen(&ret_tokens)
}
