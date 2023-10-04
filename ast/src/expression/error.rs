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
    #[allow(dead_code)]
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

    pub fn from_cl_ln<T: Sized>(error_type: ErrorType, v: &T) -> Error
    where
        T: cl_ln::ClLn,
    {
        Error {
            error_type,
            cl_start: v.cl_start(),
            cl_end: v.cl_end(),
            ln_start: v.ln_start(),
            ln_end: v.ln_end(),
        }
    }
}

impl cl_ln::ClLn for Error {
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
