use super::error;
use super::exp;
use super::ExpressionToken;
use crate::node;
use crate::node::expression::*;
use cl_ln::ClLn;

// Parses all single data units. This is the smallest unit of an expression. For example literals,
// function calls, property access, etc.
//
// # Example:
// ```
// 3
// 4.5
// "Hello world"
// my_function()
// my_variable
// std::print(*already evaluated*)[0].property
pub(super) fn all(tokens: &[ExpressionToken]) -> Result<node::expression::All, error::Error> {
    if tokens.len() == 0 {
        // Should never get here
        panic!("Cannot parse empty expression");
    }

    if tokens.len() >= 1 {
        if let ExpressionToken::Token(t) = &tokens[0] {
            match t.token_type {
                lexer::TokenType::Not => {
                    let value = &tokens[1..];
                    let value = all(value)?;

                    let cl_ln = cl_ln::combine(&tokens);

                    return Ok(All::BooleanLogic {
                        value: BooleanLogic::Not {
                            value: Box::new(All::from(value)),
                            cl_ln,
                        },
                        cl_ln,
                    });
                }
                lexer::TokenType::Sub => {
                    let value = &tokens[1..];
                    let value = all(value)?;

                    let cl_ln = cl_ln::combine(&tokens);

                    return Ok(All::Arithmetic {
                        value: Arithmetic::Neg {
                            value: Box::new(All::from(value)),
                            cl_ln,
                        },
                        cl_ln,
                    });
                }
                _ => {}
            }
        }
    }

    macro_rules! literal {
        ($scope:ident::$type:ident, $t:expr, $v:expr) => {
            let cl_ln = $t.cl_ln();

            let l = $scope::$type { value: $v, cl_ln };

            let expression = All::SingleDataUnit {
                value: SingleDataUnit::Literal { literal: l, cl_ln },
                cl_ln,
            };

            return all(&[&[ExpressionToken::Expression(expression)], &tokens[1..]].concat());
        };
    }

    match &tokens[0] {
        ExpressionToken::Expression(e) => {
            if tokens.len() == 1 {
                return Ok(e.clone());
            } else {
                if let ExpressionToken::Token(t) = &tokens[1] {
                    match &t.token_type {
                        lexer::TokenType::Dot => {
                            return parse_property_access(e.clone(), &tokens[1..]);
                        }
                        lexer::TokenType::SquareOpen => {
                            return parse_array_access(e.clone(), &tokens[1..]);
                        }
                        lexer::TokenType::ParenOpen => {
                            return parse_function_call(e.clone(), &tokens[1..]);
                        }
                        lexer::TokenType::CurlyOpen => {
                            // TODO: Implement struct init
                            unimplemented!();
                        }
                        _ => {
                            return Err(error::Error::from_cl_ln(
                                error::ErrorType::UnexpectedToken(t.token_type.clone()),
                                t,
                            ));
                        }
                    }
                }
            }
        }
        ExpressionToken::Token(t) => match &t.token_type {
            lexer::TokenType::FloatLiteral(f) => {
                literal!(Literal::Float, t, *f);
            }
            lexer::TokenType::IntegerLiteral(i) => {
                literal!(Literal::Integer, t, *i);
            }
            lexer::TokenType::StringLiteral(s) => {
                literal!(Literal::String, t, s.to_string());
            }
            lexer::TokenType::BooleanLiteral(b) => {
                literal!(Literal::Boolean, t, *b);
            }
            lexer::TokenType::SquareOpen => {
                // TODO: Implement array
                unimplemented!();
            }
            lexer::TokenType::Identifier(_) => {
                return parse_identifier(tokens);
            }
            _ => {
                return Err(error::Error::from_cl_ln(
                    error::ErrorType::UnexpectedToken(t.token_type.clone()),
                    t,
                ));
            }
        },
    };

    panic!("Should never get here");
}

fn parse_array_access(
    expression: node::expression::All,
    tokens: &[ExpressionToken],
) -> Result<node::expression::All, error::Error> {
    if tokens.len() == 0 {
        panic!("Cannot parse empty expression");
    }

    match &tokens[0] {
        ExpressionToken::Token(lexer::Token {
            token_type: lexer::TokenType::SquareOpen,
            ..
        }) => {}
        _ => panic!("First token must be a ["),
    }

    let mut curly_count = 0;
    let mut square_count = 1;
    let mut paren_count = 0;

    for i in 1..tokens.len() {
        if let ExpressionToken::Token(t) = &tokens[i] {
            match &t.token_type {
                lexer::TokenType::ParenOpen => paren_count += 1,
                lexer::TokenType::ParenClose => paren_count -= 1,
                lexer::TokenType::CurlyOpen => curly_count += 1,
                lexer::TokenType::CurlyClose => curly_count -= 1,
                lexer::TokenType::SquareOpen => square_count += 1,
                lexer::TokenType::SquareClose => {
                    if square_count == 1 {
                        if curly_count != 0 || paren_count != 0 {
                            return Err(error::Error::from_cl_ln(
                                error::ErrorType::UnexpectedCloseSquare,
                                t,
                            ));
                        }

                        let array_access_tokens = &tokens[1..i];

                        if array_access_tokens.len() == 0 {
                            return Err(error::Error::from_cl_ln(
                                error::ErrorType::EmptyExpression,
                                &cl_ln::combine(&tokens[1..=i]),
                            ));
                        }

                        let cl_ln = cl_ln::combine(
                            &[
                                &[ExpressionToken::Expression(expression.clone())],
                                &tokens[..=i],
                            ]
                            .concat(),
                        );

                        let expression = All::SingleDataUnit {
                            cl_ln,
                            value: SingleDataUnit::ArrayAccess {
                                cl_ln,
                                array: Box::new(expression.clone()),
                                index: Box::new(exp::gen(array_access_tokens)?),
                            },
                        };

                        return all(&[
                            &[ExpressionToken::Expression(expression)],
                            &tokens[i + 1..],
                        ]
                        .concat());
                    }
                    square_count -= 1;
                }
                _ => {}
            }
        }
    }

    match &tokens[0] {
        ExpressionToken::Token(t) => Err(error::Error::from_cl_ln(
            error::ErrorType::SquareNotClosed,
            t,
        )),
        _ => panic!("First token must be a ["),
    }
}

/// Parses property access
///
/// Assumes that first token is .
///
/// If there are more tokens after the property access, then the function will call `all` recessively again
///
/// # Example
/// ```
/// test.test
/// (3 + 1).example
/// ```
fn parse_property_access(
    expression: node::expression::All,
    tokens: &[ExpressionToken],
) -> Result<node::expression::All, error::Error> {
    if tokens.len() == 0 {
        panic!("Cannot parse empty expression");
    }

    match &tokens[0] {
        ExpressionToken::Token(lexer::Token {
            token_type: lexer::TokenType::Dot,
            ..
        }) => {}
        _ => panic!("First token must be a dot"),
    }

    if tokens.len() == 1 {
        let cl_ln = cl_ln::combine(&[ExpressionToken::Expression(expression), tokens[0].clone()]);

        return Err(error::Error::from_cl_ln(
            error::ErrorType::NoPropertyOnAccess,
            &cl_ln,
        ));
    }
    if let ExpressionToken::Token(t) = &tokens[1] {
        match &t.token_type {
            lexer::TokenType::Identifier(i) => {
                let cl_ln = cl_ln::combine(
                    &[
                        &[ExpressionToken::Expression(expression.clone())],
                        &tokens[..=1],
                    ]
                    .concat(),
                );

                let expression = All::SingleDataUnit {
                    cl_ln,
                    value: SingleDataUnit::PropertyAccess {
                        object: Box::new(expression.clone()),
                        property: i.to_string(),
                        cl_ln,
                    },
                };

                return all(&[&[ExpressionToken::Expression(expression)], &tokens[2..]].concat());
            }
            _ => {
                return Err(error::Error::from_cl_ln(
                    error::ErrorType::UnexpectedToken(t.token_type.clone()),
                    t,
                ));
            }
        }
    } else {
        Err(error::Error::from_cl_ln(
            error::ErrorType::UnexpectedExpression,
            &tokens[1],
        ))
    }
}

fn parse_function_call(
    expression: node::expression::All,
    tokens: &[ExpressionToken],
) -> Result<node::expression::All, error::Error> {
    if tokens.len() == 0 {
        panic!("Cannot parse empty expression");
    }

    match &tokens[0] {
        ExpressionToken::Token(lexer::Token {
            token_type: lexer::TokenType::ParenOpen,
            ..
        }) => {}
        _ => panic!("First token must be a ("),
    }

    let mut curly_count = 0;
    let mut square_count = 0;
    let mut paren_count = 1;

    let mut start = 1;
    let mut end = tokens.len();
    let mut args = vec![];

    for i in 1..tokens.len() {
        if let ExpressionToken::Token(t) = &tokens[i] {
            match &t.token_type {
                lexer::TokenType::ParenOpen => paren_count += 1,
                lexer::TokenType::ParenClose => {
                    paren_count -= 1;

                    if curly_count != 0 || paren_count != 0 {
                        return Err(error::Error::from_cl_ln(
                            error::ErrorType::UnexpectedCloseParen,
                            t,
                        ));
                    }

                    if paren_count == 0 {
                        end = i;
                        break;
                    }
                }
                lexer::TokenType::CurlyOpen => curly_count += 1,
                lexer::TokenType::CurlyClose => curly_count -= 1,
                lexer::TokenType::SquareOpen => square_count += 1,
                lexer::TokenType::SquareClose => square_count -= 1,
                lexer::TokenType::Comma => {
                    if paren_count != 1 || curly_count != 0 || square_count != 0 {
                        continue;
                    }

                    let exp_tokens = &tokens[start..i];

                    if exp_tokens.len() == 0 {
                        return Err(error::Error::from_cl_ln(
                            error::ErrorType::EmptyExpression,
                            &cl_ln::combine(&tokens[i - 1..=i]),
                        ));
                    }

                    let expression = exp::gen(exp_tokens)?;
                    args.push(expression);
                    start = i + 1;
                }

                _ => {}
            }
        }
    }

    if paren_count != 0 || curly_count != 0 || square_count != 0 {
        if paren_count != 0 {
            return Err(error::Error::from_cl_ln(
                error::ErrorType::ParenNotClosed,
                &tokens[0],
            ));
        } else {
            panic!("Parenthesis not closed, should be handled by the loop above");
        }
    }

    let last_arg_tokens = &tokens[start..end];
    if last_arg_tokens.len() != 0 {
        args.push(exp::gen(last_arg_tokens)?);
    }

    let all_tokens = &[
        &[ExpressionToken::Expression(expression.clone())],
        &tokens[..=end],
    ]
    .concat();

    let expression = All::SingleDataUnit {
        value: SingleDataUnit::FunctionCall {
            function: Box::new(expression.clone()),
            arguments: args,
            cl_ln: cl_ln::combine(all_tokens),
        },
        cl_ln: cl_ln::combine(all_tokens),
    };

    return all(&[
        &[ExpressionToken::Expression(expression)],
        &tokens[end + 1..],
    ]
    .concat());
}

/// Parses an identifier
///
/// Assumes that first token is an identifier
///
/// # Example
/// ```
/// my_function
/// std::print
/// std::print::println
/// ```
///
fn parse_identifier(tokens: &[ExpressionToken]) -> Result<node::expression::All, error::Error> {
    let mut peek_iter = tokens.iter().enumerate().peekable();

    let mut vals: Vec<String> = vec![];

    let mut was_last_double_colon = false;

    while let Some((i, t)) = peek_iter.next() {
        let t = match t {
            ExpressionToken::Token(t) => t,
            ExpressionToken::Expression(e) => {
                return Err(error::Error::from_cl_ln(
                    error::ErrorType::UnexpectedExpression,
                    e,
                ));
            }
        };

        match &t.token_type {
            lexer::TokenType::Identifier(ident) => {
                vals.push(ident.to_string());

                if i != 0 && !was_last_double_colon {
                    return Err(error::Error::from_cl_ln(
                        error::ErrorType::UnexpectedToken(lexer::TokenType::Identifier(
                            ident.to_string(),
                        )),
                        t,
                    ));
                }

                was_last_double_colon = false;
            }
            lexer::TokenType::DoubleColon => {
                was_last_double_colon = true;

                if i == 0 {
                    panic!("Double colon cannot be first token in an identifier");
                }

                if peek_iter.peek().is_none() {
                    return Err(error::Error::from_cl_ln(
                        error::ErrorType::UnexpectedToken(lexer::TokenType::DoubleColon),
                        t,
                    ));
                }

                let nt = peek_iter.peek().unwrap().1;

                match nt {
                    ExpressionToken::Token(lexer::Token {
                        token_type: lexer::TokenType::Identifier(_),
                        ..
                    }) => {}
                    ExpressionToken::Token(t) => {
                        return Err(error::Error::from_cl_ln(
                            error::ErrorType::UnexpectedToken(t.token_type.clone()),
                            t,
                        ));
                    }
                    ExpressionToken::Expression(e) => {
                        return Err(error::Error::from_cl_ln(
                            error::ErrorType::UnexpectedExpression,
                            e,
                        ));
                    }
                };
            }
            _ => {
                break;
            }
        };
    }

    if vals.len() == 0 {
        panic!("Identifier is empty");
    }

    let namespace = vals[..vals.len() - 1].to_vec();
    let identifier = vals[vals.len() - 1].clone();

    let end = vals.len() * 2 - 1;

    let cl_ln = cl_ln::combine(&tokens[..end]);

    let expression = All::SingleDataUnit {
        value: SingleDataUnit::Identifier {
            namespace,
            identifier,
            cl_ln,
        },
        cl_ln,
    };

    return all(&[&[ExpressionToken::Expression(expression)], &tokens[end..]].concat());
}
