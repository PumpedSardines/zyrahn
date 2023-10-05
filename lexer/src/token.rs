#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    // Keywords
    Function,
    Const,
    Var,
    If,
    Else,
    Return,
    Break,
    Continue,
    While,
    Struct,
    Out,
    Namespace,

    Integer,
    Float,
    Boolean,
    String,

    // Symbols
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    ParenOpen,
    ParenClose,
    CurlyOpen,
    CurlyClose,
    SquareOpen,
    SquareClose,

    RightArrow,

    Semicolon,
    Colon,
    DoubleColon,
    Comma,
    Dot,

    Assign,
    AddAssign,
    SubAssign,

    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Or,
    And,
    Not,

    // Literals
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Function => write!(f, "fnc"),
            TokenType::Const => write!(f, "cst"),
            TokenType::Var => write!(f, "var"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),

            TokenType::Add => write!(f, "+"),
            TokenType::Sub => write!(f, "-"),
            TokenType::Mul => write!(f, "*"),
            TokenType::Div => write!(f, "/"),
            TokenType::Mod => write!(f, "%"),
            TokenType::Pow => write!(f, "**"),

            TokenType::ParenOpen => write!(f, "("),
            TokenType::ParenClose => write!(f, ")"),
            TokenType::CurlyOpen => write!(f, "{{"),
            TokenType::CurlyClose => write!(f, "}}"),
            TokenType::SquareOpen => write!(f, "["),
            TokenType::SquareClose => write!(f, "]"),

            TokenType::RightArrow => write!(f, "->"),

            TokenType::Semicolon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::DoubleColon => write!(f, "::"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),

            TokenType::Assign => write!(f, "="),
            TokenType::AddAssign => write!(f, "+="),
            TokenType::SubAssign => write!(f, "-="),

            TokenType::Equal => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::LessThan => write!(f, "<"),
            TokenType::LessThanOrEqual => write!(f, "<="),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::GreaterThanOrEqual => write!(f, ">="),
            TokenType::Or => write!(f, "||"),
            TokenType::And => write!(f, "&&"),
            TokenType::Not => write!(f, "!"),

            TokenType::StringLiteral(string) => write!(f, "\"{}\"", string),
            TokenType::Identifier(string) => write!(f, "{}", string),
            TokenType::IntegerLiteral(num) => write!(f, "{}", num),
            TokenType::FloatLiteral(num) => {
                if *num == num.round() {
                    write!(f, "{}.0", num)
                } else {
                    write!(f, "{}", num)
                }
            }
            TokenType::BooleanLiteral(boolean) => write!(f, "{}", boolean),
            _ => todo!(),
        }
    }
}

impl TokenType {
    /// Returns if it's the same token type, ignoring the value
    fn shallow_eq(&self, other: &TokenType) -> bool {
        match self {
            TokenType::Identifier(_) => match other {
                TokenType::Identifier(_) => true,
                _ => false,
            },
            TokenType::IntegerLiteral(_) => match other {
                TokenType::IntegerLiteral(_) => true,
                _ => false,
            },
            TokenType::FloatLiteral(_) => match other {
                TokenType::FloatLiteral(_) => true,
                _ => false,
            },
            TokenType::StringLiteral(_) => match other {
                TokenType::StringLiteral(_) => true,
                _ => false,
            },
            TokenType::BooleanLiteral(_) => match other {
                TokenType::BooleanLiteral(_) => true,
                _ => false,
            },
            _ => self == other,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub ln_start: usize,
    pub cl_start: usize,
    pub ln_end: usize,
    pub cl_end: usize,
}

impl cl_ln::ClLn for Token {
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
