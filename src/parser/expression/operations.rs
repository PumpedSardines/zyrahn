use super::*;
use parser::node::expression;

const ORDER_OF_OPERATIONS: &'static [&'static [lexer::TokenType]] = {
    use lexer::TokenType::*;

    &[
        &[Pow],
        &[Mul, Div],
        &[Add, Sub],
        &[Mod],
        &[
            Equal,
            NotEqual,
            LessThan,
            LessThanOrEqual,
            GreaterThan,
            GreaterThanOrEqual,
        ],
        &[Or, And],
    ]
};

fn is_op(token_type: &lexer::TokenType) -> bool {
    use lexer::TokenType::*;

    match token_type {
        Add | Sub | Mul | Div | Mod | Or | And | Equal | NotEqual | LessThan | LessThanOrEqual
        | GreaterThan | GreaterThanOrEqual | Not => true,
        _ => false,
    }
}

/// Parses comparison operators with parentheses already calculated
///
/// # Example
/// 3 + 4 * 5 - (6 + 7)
///
/// - (6 + 7) is presumed to be pre-calculated
/// - Go through all other operators in order of operations and send the values in between to
/// single_data_unit parsing
pub(super) fn gen(
    tokens: &[ExpressionToken],
) -> Result<Node<expression::All>, error::Error<error::ParserErrorType>> {
    for order_of_operations in ORDER_OF_OPERATIONS.iter().rev() {
        let itr = tokens.iter();

        let mut curly_count = 0;
        let mut square_count = 0;
        let mut paren_count = 0;

        for (i, token) in itr.enumerate() {
            if let ExpressionToken::Token(token) = token {
                if token.token_type == lexer::TokenType::Sub {
                    if i == 0 {
                        continue;
                    }

                    if let ExpressionToken::Token(t) = &tokens[i - 1] {
                        if is_op(&t.token_type) {
                            continue;
                        }
                    }
                }

                match &token.token_type {
                    lexer::TokenType::ParenOpen => paren_count += 1,
                    lexer::TokenType::ParenClose => paren_count -= 1,
                    lexer::TokenType::CurlyOpen => curly_count += 1,
                    lexer::TokenType::CurlyClose => curly_count -= 1,
                    lexer::TokenType::SquareOpen => square_count += 1,
                    lexer::TokenType::SquareClose => square_count -= 1,
                    _ => {}
                }

                if curly_count < 0 || square_count < 0 || paren_count < 0 {
                    return Err(error::Error::from_cl_ln(
                        {
                            if curly_count < 0 {
                                error::ParserErrorType::UnexpectedCloseCurly
                            } else if square_count < 0 {
                                error::ParserErrorType::UnexpectedCloseSquare
                            } else {
                                error::ParserErrorType::UnexpectedCloseParen
                            }
                        },
                        token,
                    ));
                }

                if curly_count != 0 || square_count != 0 || paren_count != 0 {
                    continue;
                }

                if order_of_operations.contains(&&token.token_type) {
                    let left = &tokens[..i];
                    let right = &tokens[i + 1..];

                    macro_rules! matcher {
                        ($s:ident::$x:ident) => {{
                            if left.len() == 0 || right.len() == 0 {
                                return Err(error::Error::from_cl_ln(
                                    error::ParserErrorType::CannotPerformOperationOnEmpty(
                                        lexer::TokenType::$x,
                                    ),
                                    token,
                                ));
                            }

                            let left = gen(left)?;
                            let right = gen(right)?;

                            let cl_ln = cl_ln::combine(&tokens);

                            Node::from_cl_ln(
                                expression::All::$s {
                                    value: expression::$s::$x {
                                        left: Box::new(left),
                                        right: Box::new(right),
                                    },
                                },
                                &cl_ln,
                            )
                        }};
                        ($x:ident) => {
                            lexer::TokenType::$x
                        };
                    }

                    return Ok(match token.token_type {
                        matcher!(Add) => matcher!(Arithmetic::Add),
                        matcher!(Pow) => matcher!(Arithmetic::Pow),
                        matcher!(Sub) => matcher!(Arithmetic::Sub),
                        matcher!(Mul) => matcher!(Arithmetic::Mul),
                        matcher!(Div) => matcher!(Arithmetic::Div),
                        matcher!(Mod) => matcher!(Arithmetic::Mod),
                        matcher!(Or) => matcher!(BooleanLogic::Or),
                        matcher!(And) => matcher!(BooleanLogic::And),
                        matcher!(Equal) => matcher!(Cmp::Equal),
                        matcher!(NotEqual) => matcher!(Cmp::NotEqual),
                        matcher!(LessThan) => matcher!(Cmp::LessThan),
                        matcher!(LessThanOrEqual) => matcher!(Cmp::LessThanOrEqual),
                        matcher!(GreaterThan) => matcher!(Cmp::GreaterThan),
                        matcher!(GreaterThanOrEqual) => matcher!(Cmp::GreaterThanOrEqual),

                        _ => unreachable!(),
                    });
                }
            }
        }
    }

    single_data_unit::all(tokens)
}
