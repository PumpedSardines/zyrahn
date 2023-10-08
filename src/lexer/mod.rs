//! The lexer for zyrahn.
//!
//! What is a lexer?
//! A lexer is a program that takes in a string of code and turns it into a list of tokens. A
//! tokens is a representation of a piece of code. For example, the string "3 + 3" would be turned
//! into the tokens [IntegerLiteral(3), Add, IntegerLiteral(3)]. This is useful because it makes
//! it easier to create the abstract syntax tree (AST) later on.

use crate::*;

mod token;
pub use token::*;

const SYMBOLS: &[&'static str] = &[
    "+", "-", "*", "/", "(", ")", "[", "]", "{", "}", "->", ";", ":", "::", ",", ".", "=", "+=",
    "-=", "==", "!=", "<", ">", "<=", ">=", "||", "&&", "!", "%", "**",
];

fn is_symbol_char(char: char) -> bool {
    SYMBOLS.iter().any(|s| s.chars().any(|c| c == char))
}

fn is_word_char(char: char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_number_char(char: char) -> bool {
    matches!(char, '0'..='9')
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Eval {
    Word,
    Symbol,
    Number,
    String,
}

impl Eval {
    fn from_char(char: char) -> Option<Eval> {
        if is_symbol_char(char) {
            Some(Eval::Symbol)
        } else if is_word_char(char) {
            Some(Eval::Word)
        } else if is_number_char(char) {
            Some(Eval::Number)
        } else {
            None
        }
    }
}

/// Turns a string of code into a list of tokens.
///
/// # Examples
/// ```
/// use zyrahn::lexer;
///
/// let tokens = lexer::tokenize("3 + 3");
///
/// if let Err(e) = tokens {
///     println!("{}", e);
///     return;
/// }
///
/// let tokens = tokens.unwrap();
///
/// println!("{:?}", tokens); // [IntegerLiteral(3), Add, IntegerLiteral(3)]
/// ```
pub fn tokenize(code: &str) -> Result<Vec<Token>, error::Error<error::LexerErrorType>> {
    let mut tokens = vec![];

    let lines = code.split('\n');

    for (ln, line) in lines.enumerate() {
        let mut cur_eval: Option<Eval> = None;
        let mut cur_text = String::new();
        let mut cl_start = 0;
        let mut chars = line.chars().enumerate().peekable();

        let mut is_esc = false;

        while let Some((cl, c)) = chars.next() {
            let c = c.clone();

            if c == '"' {
                if is_esc {
                    cur_text.push(c);
                    is_esc = false;
                    continue;
                }

                if cur_eval == Some(Eval::String) {
                    tokens.push(parse_token(&cur_text, Eval::String, ln, cl_start, cl)?);
                    cur_eval = None;
                    cur_text.clear();
                    continue;
                }

                cur_eval = Some(Eval::String);
                cur_text.clear();
                cl_start = cl;

                continue;
            }

            if cur_eval == Some(Eval::String) {
                if c == '\\' {
                    if is_esc {
                        cur_text.push(c);
                        is_esc = false;
                    } else {
                        is_esc = true;
                    }
                    continue;
                } else {
                    is_esc = false;
                    cur_text.push(c);
                }

                continue;
            }

            if c == '/' {
                if let Some((_, c2)) = chars.peek() {
                    if *c2 == '/' {
                        break;
                    }
                }
            }

            let c_is_white_space = c == ' ' || c == '\t';

            if !c_is_white_space
                && cur_eval == Some(Eval::Number)
                && Eval::from_char(c) == Some(Eval::Symbol)
            {
                if let Ok(token) = parse_token(&format!("{}", c), Eval::Symbol, ln, cl, cl) {
                    if token.token_type == TokenType::Dot {
                        cur_text.push(c);

                        continue;
                    }
                }
            }

            if c_is_white_space
                || (Eval::from_char(c) != cur_eval && Eval::from_char(c) != Some(Eval::Symbol))
            {
                if let Some(e) = cur_eval {
                    tokens.push(parse_token(&cur_text, e, ln, cl_start, cl - 1)?);

                    cur_eval = Eval::from_char(c);
                    cur_text.clear();
                    if cur_eval != None {
                        cur_text.push(c);
                    }

                    continue;
                }

                cur_eval = None;
            }

            if !c_is_white_space {
                match Eval::from_char(c) {
                    Some(Eval::Symbol) => {
                        if let Some(e) = cur_eval {
                            tokens.push(parse_token(&cur_text, e, ln, cl_start, cl - 1)?);
                            cur_text.clear();
                        }

                        let symbol = format!(
                            "{}{}",
                            c,
                            chars
                                .peek()
                                .map(|(_, c)| c.to_string())
                                .unwrap_or("SOMETHING_THAT_WILL_NEVER_MATCH".to_string())
                        );

                        if let Ok(token) = parse_token(&symbol, Eval::Symbol, ln, cl, cl + 1) {
                            tokens.push(token);
                            chars.next();
                        } else {
                            tokens.push(parse_token(&format!("{}", c), Eval::Symbol, ln, cl, cl)?);
                        }

                        cur_eval = None;
                    }
                    None => {
                        return Err(error::Error::new(
                            error::LexerErrorType::UnexpectedSymbol(c.to_string()),
                            ln + 1,
                            cl + 1,
                            ln + 1,
                            cl + 1,
                        ));
                    }
                    _ => {
                        if Eval::from_char(c) != cur_eval {
                            cur_eval = Eval::from_char(c);
                            cl_start = cl;
                            cur_text.clear();
                        }
                        cur_text.push(c);
                    }
                }
            }
        }

        if cur_eval == Some(Eval::String) {
            let ln = ln + 1;
            let cl_start = cl_start + 1;

            return Err(error::Error::new(
                error::LexerErrorType::NonTerminatedString,
                ln,
                cl_start,
                ln,
                line.len(),
            ));
        }

        if let Some(e) = cur_eval {
            tokens.push(parse_token(&cur_text, e, ln, cl_start, line.len() - 1)?);
        }
    }

    Ok(tokens)
}

fn parse_token(
    word: &str,
    eval: Eval,
    ln: usize,
    cl_start: usize,
    cl_end: usize,
) -> Result<Token, error::Error<error::LexerErrorType>> {
    let token_type = match eval {
        Eval::Word => match word {
            "fnc" => TokenType::Function,
            "cst" => TokenType::Const,
            "var" => TokenType::Var,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "ret" => TokenType::Return,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "while" => TokenType::While,
            "struct" => TokenType::Struct,
            "int" => TokenType::Integer,
            "flt" => TokenType::Float,
            "bln" => TokenType::Boolean,
            "str" => TokenType::String,
            "true" => TokenType::BooleanLiteral(true),
            "false" => TokenType::BooleanLiteral(false),
            "out" => TokenType::Out,
            "ns" => TokenType::Namespace,
            "__COMPILER_SET_CUSTOM_PRE_DEFINED" => {
                TokenType::CompilerSetCustomCodePreDefined("".to_string())
            }
            string => TokenType::Identifier(string.to_string()),
        },
        Eval::Symbol => {
            if !SYMBOLS.iter().any(|s| *s == word) {
                let ln = ln + 1;
                let cl_start = cl_start + 1;
                let cl_end = cl_end + 1;

                return Err(error::Error::new(
                    error::LexerErrorType::UnexpectedSymbol(word.to_string()),
                    ln,
                    cl_start,
                    ln,
                    cl_end,
                ));
            }

            match word {
                "+" => TokenType::Add,
                "-" => TokenType::Sub,
                "*" => TokenType::Mul,
                "**" => TokenType::Pow,
                "%" => TokenType::Mod,
                "/" => TokenType::Div,
                "(" => TokenType::ParenOpen,
                ")" => TokenType::ParenClose,
                "{" => TokenType::CurlyOpen,
                "}" => TokenType::CurlyClose,
                "[" => TokenType::SquareOpen,
                "]" => TokenType::SquareClose,
                "->" => TokenType::RightArrow,
                ";" => TokenType::Semicolon,
                "::" => TokenType::DoubleColon,
                ":" => TokenType::Colon,
                "," => TokenType::Comma,
                "." => TokenType::Dot,
                "=" => TokenType::Assign,
                "+=" => TokenType::AddAssign,
                "-=" => TokenType::SubAssign,
                "==" => TokenType::Equal,
                "!=" => TokenType::NotEqual,
                "<" => TokenType::LessThan,
                "<=" => TokenType::LessThanOrEqual,
                ">" => TokenType::GreaterThan,
                ">=" => TokenType::GreaterThanOrEqual,
                "||" => TokenType::Or,
                "&&" => TokenType::And,
                "!" => TokenType::Not,
                _ => unreachable!(),
            }
        }
        Eval::Number => {
            if let Ok(num) = word.parse::<i64>() {
                TokenType::IntegerLiteral(num)
            } else if let Ok(num) = word.parse::<f64>() {
                TokenType::FloatLiteral(num)
            } else {
                let ln = ln + 1;
                let cl_start = cl_start + 1;
                let cl_end = cl_end + 1;

                return Err(error::Error::new(
                    error::LexerErrorType::InvalidNumber(word.to_string()),
                    ln,
                    cl_start,
                    ln,
                    cl_end,
                ));
            }
        }
        Eval::String => TokenType::StringLiteral(word.to_string()),
    };

    let ln = ln + 1;
    let cl_start = cl_start + 1;
    let cl_end = cl_end + 1;

    Ok(Token {
        token_type,
        ln_start: ln,
        cl_start,
        ln_end: ln,
        cl_end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens(tokens: Vec<Token>, token_types: Vec<TokenType>) {
        if tokens.len() != token_types.len() {
            panic!(
                "Tokens and token types are not the same length: {} != {}",
                tokens.len(),
                token_types.len()
            );
        }

        for i in 0..tokens.len() {
            assert_eq!(tokens[i].token_type, token_types[i]);
        }
    }
}
