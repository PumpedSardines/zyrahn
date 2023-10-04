use crate::node;

#[derive(Debug)]
pub enum ErrorType {
    UnexpectedCloseParen,
    UnexpectedCloseCurly,
    UnexpectedCloseSquare,
    SquareNotClosed,
    CurlyNotClosed,
    ParenNotClosed,
    UnexpectedExpression,
    UnclosedExpression,
    EmptyExpression,
    CannotPerformOperationOnEmpty(lexer::TokenType),
    // When a property is being accessed, but no property key is found
    // e.g. `value.`
    NoPropertyOnAccess,
    UnexpectedToken(lexer::TokenType),
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ErrorType as ET;

        match self {
            ET::UnexpectedCloseParen => write!(f, "Unexpected close ')'"),
            ET::UnexpectedCloseCurly => write!(f, "Unexpected close '}}'"),
            ET::UnexpectedCloseSquare => write!(f, "Unexpected close ']'"),
            ET::SquareNotClosed => write!(f, "Square bracket not closed"),
            ET::CurlyNotClosed => write!(f, "Curly bracket not closed"),
            ET::ParenNotClosed => write!(f, "Parenthesis not closed"),
            ET::UnexpectedToken(token_type) => write!(f, "Unexpected token '{}'", token_type),
            ET::UnexpectedExpression => write!(f, "Unexpected expression"),
            ET::UnclosedExpression => write!(f, "Unclosed expression"),
            ET::CannotPerformOperationOnEmpty(token_type) => {
                write!(
                    f,
                    "Cannot perform operation '{}' on empty expression ",
                    token_type
                )
            }
            ET::EmptyExpression => write!(f, "Empty expression"),
            ET::NoPropertyOnAccess => write!(f, "No property after '.' when accessing property"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    error_type: ErrorType,
    cl_start: usize,
    #[allow(dead_code)]
    cl_end: usize,
    ln_start: usize,
    #[allow(dead_code)]
    ln_end: usize,
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(
        error_type: ErrorType,
        cl_start: usize,
        cl_end: usize,
        ln_start: usize,
        ln_end: usize,
    ) -> Error {
        Error {
            error_type,
            cl_start,
            cl_end,
            ln_start,
            ln_end,
        }
    }

    pub fn from_cl_ln(error_type: ErrorType, token: node::ClLn) -> Error {
        Error {
            error_type,
            cl_start: token.1,
            cl_end: token.3,
            ln_start: token.0,
            ln_end: token.2,
        }
    }

    pub fn from_token(error_type: ErrorType, token: &lexer::Token) -> Error {
        Error {
            error_type,
            cl_start: token.cl_start,
            cl_end: token.cl_end,
            ln_start: token.ln_start,
            ln_end: token.ln_end,
        }
    }

    pub fn from_many_tokens(error_type: ErrorType, tokens: &[lexer::Token]) -> Error {
        let min_cl = tokens.iter().map(|t| t.cl_start).min().unwrap();
        let max_cl = tokens.iter().map(|t| t.cl_end).max().unwrap();
        let min_ln = tokens.iter().map(|t| t.ln_start).min().unwrap();
        let max_ln = tokens.iter().map(|t| t.ln_end).max().unwrap();

        Error {
            error_type,
            cl_start: min_cl,
            cl_end: max_cl,
            ln_start: min_ln,
            ln_end: max_ln,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cl = {
            if self.cl_start == self.cl_end {
                format!("{}", self.cl_start)
            } else {
                format!("{}-{}", self.cl_start, self.cl_end)
            }
        };

        let ln = {
            if self.ln_start == self.ln_end {
                format!("{}", self.ln_start)
            } else {
                format!("{}-{}", self.ln_start, self.ln_end)
            }
        };

        write!(f, "ln: {} cl: {} - {}", ln, cl, self.error_type)
    }
}
