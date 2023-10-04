#[derive(Debug)]
pub enum ErrorType {
    UnexpectedSymbol(String),
    InvalidNumber(String),
    NonTerminatedString,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ErrorType as ET;

        match self {
            ET::UnexpectedSymbol(s) => write!(f, "Unexpected symbol: '{}'", s),
            ET::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ET::NonTerminatedString => write!(f, "Non terminated string"),
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
        ln_start: usize,
        cl_start: usize,
        ln_end: usize,
        cl_end: usize,
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
        Error::new(
            error_type,
            v.cl_start(),
            v.cl_end(),
            v.ln_start(),
            v.ln_end(),
        )
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
