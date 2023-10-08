use crate::*;

#[derive(Debug, Clone)]
pub enum StaticAnalyzerErrorType {
    TypeMismatchOp(lexer::TokenType, common::Type, common::Type),
    TypeMismatchAssign(common::Type, common::Type),
    OperationNotSupportedNeg(common::Type),
    OperationNotSupportedNot(common::Type),
    OperationNotSupported(lexer::TokenType, common::Type),
    VariableNotDefined(String, Vec<String>),
    FunctionNotDefined(String, Vec<String>),
    FunctionArgumentMismatch(String, Vec<String>, Vec<(bool, common::Type)>),
    CannotCallNonFunction,
    FeatureNotImplemented(String),
    CompilerCustomCodePreDefined,
    CannotUseNonIdentifierAsOutArgument,
}

impl std::fmt::Display for StaticAnalyzerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use StaticAnalyzerErrorType as ET;

        match self {
            ET::CannotUseNonIdentifierAsOutArgument => {
                write!(f, "Cannot use non-identifier as out argument")
            }
            ET::CompilerCustomCodePreDefined => {
                write!(f, "Compiler set custom code pre-defined error")
            }
            ET::TypeMismatchAssign(left, right) => {
                write!(f, "Type mismatch for assignment: {} != {}", left, right)
            }
            ET::TypeMismatchOp(token_type, left, right) => {
                write!(
                    f,
                    "Type mismatch for token '{}': {} != {}",
                    token_type, left, right
                )
            }
            ET::OperationNotSupportedNeg(value) => {
                write!(f, "Negation is not supported for type '{}'", value)
            }
            ET::OperationNotSupportedNot(value) => {
                write!(f, "Not is not supported for type '{}'", value)
            }
            ET::OperationNotSupported(token_type, value) => {
                write!(f, "Cannot use '{}' for type '{}'", token_type, value)
            }
            ET::VariableNotDefined(name, ns) => {
                let variable_name = {
                    if ns.len() != 0 {
                        format!("{}::{}", ns.join("::"), name)
                    } else {
                        name.clone()
                    }
                };

                write!(f, "Variable '{}' is not declared", variable_name)
            }
            ET::FunctionNotDefined(name, ns) => {
                let function_name = {
                    if ns.len() != 0 {
                        format!("{}::{}", ns.join("::"), name)
                    } else {
                        name.clone()
                    }
                };

                write!(f, "Function '{}' is not declared", function_name)
            }
            ET::FunctionArgumentMismatch(name, ns, args) => {
                let function_name = {
                    if ns.len() != 0 {
                        format!("{}::{}", ns.join("::"), name)
                    } else {
                        name.clone()
                    }
                };

                write!(
                    f,
                    "Function '{}' does not have an overload: {}({})",
                    function_name,
                    function_name,
                    args.iter()
                        .map(|(is_out, arg)| {
                            if *is_out {
                                format!("out {}", arg)
                            } else {
                                format!("{}", arg)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            ET::CannotCallNonFunction => write!(f, "Cannot call non-function"),
            ET::FeatureNotImplemented(feature) => {
                write!(
                    f,
                    "Feature '{}' not implemented yet for static_analyzer :(",
                    feature
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ParserErrorType {
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
    FeatureNotImplemented(String),

    CompilerCustomCodePreDefined,

    /// ---- Block ----
    MissingSemicolon,
    MissingIdentifier,
    StatementEndEarly,

    /// ------ Generic error -------
    UnexpectedToken(lexer::TokenType),
    UnexpectedTokenExpected(lexer::TokenType, lexer::TokenType),
}

impl std::fmt::Display for ParserErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ParserErrorType as ET;

        match self {
            ET::UnexpectedCloseParen => write!(f, "Unexpected close ')'"),
            ET::UnexpectedCloseCurly => write!(f, "Unexpected close '}}'"),
            ET::UnexpectedCloseSquare => write!(f, "Unexpected close ']'"),
            ET::SquareNotClosed => write!(f, "Square bracket not closed"),
            ET::CurlyNotClosed => write!(f, "Curly bracket not closed"),
            ET::ParenNotClosed => write!(f, "Parenthesis not closed"),
            ET::UnexpectedToken(token_type) => write!(f, "Unexpected token '{}'", token_type),
            ET::CompilerCustomCodePreDefined => {
                write!(f, "Compiler set custom code pre-defined error")
            }
            ET::UnexpectedTokenExpected(token_type, expected) => write!(
                f,
                "Unexpected token '{}', expected '{}'",
                token_type, expected
            ),
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
            ET::FeatureNotImplemented(feature) => {
                write!(f, "Feature '{}' not implemented yet for ast :(", feature)
            }
            ET::MissingSemicolon => write!(f, "Missing semicolon"),
            ET::MissingIdentifier => write!(f, "Missing identifier"),
            ET::StatementEndEarly => write!(f, "Statement ended early"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error<T>
where
    T: Sized + std::fmt::Display + std::fmt::Debug + Clone,
{
    error_type: T,
    cl_start: usize,
    #[allow(dead_code)]
    cl_end: usize,
    ln_start: usize,
    #[allow(dead_code)]
    ln_end: usize,
}

impl<T> std::error::Error for Error<T> where T: Sized + Clone + std::fmt::Display + std::fmt::Debug {}

impl<T> Error<T>
where
    T: Sized + std::fmt::Display + std::fmt::Debug + Clone,
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
    T: Sized + std::fmt::Display + std::fmt::Debug + Clone,
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
    T: Sized + std::fmt::Display + std::fmt::Debug + Clone,
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
