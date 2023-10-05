use crate::*;

#[derive(Debug)]
pub enum StaticAnalyzerErrorType {}

impl std::fmt::Display for StaticAnalyzerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use StaticAnalyzerErrorType as ET;

        match self {
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum LexerErrorType {
    UnexpectedSymbol(String),
    InvalidNumber(String),
    NonTerminatedString,
}

impl std::fmt::Display for LexerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use LexerErrorType as ET;

        match self {
            ET::UnexpectedSymbol(symbol) => write!(f, "Unexpected symbol '{}'", symbol),
            ET::InvalidNumber(number) => write!(f, "Invalid number '{}'", number),
            ET::NonTerminatedString => write!(f, "String not terminated"),
        }
    }
}

#[derive(Debug)]
pub enum AstErrorType {
    /// ---- Expression ----
    UnexpectedCloseParen,
    UnexpectedCloseCurly,
    UnexpectedCloseSquare,
    SquareNotClosed,
    // This will be used with struct inits later
    #[allow(dead_code)]
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

impl std::fmt::Display for AstErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use AstErrorType as ET;

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
pub struct Error<T>
where
    T: Sized + std::fmt::Display + std::fmt::Debug,
{
    error_type: T,
    cl_start: usize,
    #[allow(dead_code)]
    cl_end: usize,
    ln_start: usize,
    #[allow(dead_code)]
    ln_end: usize,
}

impl<T> std::error::Error for Error<T> where T: Sized + std::fmt::Display + std::fmt::Debug {}

impl<T> Error<T>
where
    T: Sized + std::fmt::Display + std::fmt::Debug,
{
    pub fn new(
        error_type: T,
        ln_start: usize,
        cl_start: usize,
        ln_end: usize,
        cl_end: usize,
    ) -> Error<T> {
        Error {
            error_type,
            cl_start,
            cl_end,
            ln_start,
            ln_end,
        }
    }

    pub fn from_cl_ln<V: Sized>(error_type: T, v: &V) -> Error<T>
    where
        V: cl_ln::ClLn,
    {
        Error::new(
            error_type,
            v.ln_start(),
            v.cl_start(),
            v.ln_end(),
            v.cl_end(),
        )
    }
}

impl<T> cl_ln::ClLn for Error<T>
where
    T: Sized + std::fmt::Display + std::fmt::Debug,
{
    fn cl_start(&self) -> usize {
        self.cl_start
    }

    fn cl_end(&self) -> usize {
        self.cl_end
    }

    fn ln_start(&self) -> usize {
        self.ln_start
    }

    fn ln_end(&self) -> usize {
        self.ln_end
    }
}

impl<T> std::fmt::Display for Error<T>
where
    T: Sized + std::fmt::Display + std::fmt::Debug,
{
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
