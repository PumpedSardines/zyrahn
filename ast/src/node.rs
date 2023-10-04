pub type ClLn = (usize, usize, usize, usize);

macro_rules! node_macro {
    (pub enum $x:ident { $($i:ident { $($k:ident : $v:ty$(,)?)+ },)+ }) => {
        #[derive(Clone, Serialize, Deserialize, Debug)]
        pub enum $x {
            $($i {
                $($k: $v,)+
                // ln_start, cl_start, ln_end, cl_end
                cl_ln: super::ClLn,
            },)+
        }

        impl $x {
            pub fn cl_ln(&self) -> super::ClLn {
                match self {
                    $($x::$i { cl_ln, .. } => *cl_ln),+
                }
            }

            pub fn cl_start(&self) -> usize {
                match self {
                    $($x::$i { cl_ln, .. } => cl_ln.1 ),+
                }
            }

            pub fn cl_end(&self) -> usize {
                match self {
                    $($x::$i { cl_ln, .. } => cl_ln.3 ),+
                }
            }

            pub fn ln_start(&self) -> usize {
                match self {
                    $($x::$i { cl_ln, .. } => cl_ln.0 ),+
                }
            }

            pub fn ln_end(&self) -> usize {
                match self {
                    $($x::$i { cl_ln, .. } => cl_ln.2 ),+
                }
            }
        }
    };
}

// pub fn cl_ln_from_token(token: &lexer::Token) -> ClLn {
//     (token.ln_start, token.cl_start, token.ln_end, token.cl_end)
// }
//
// pub fn cl_ln_from_many_token(tokens: &[lexer::Token]) -> ClLn {
//     cl_ln_from_many_cl_ln(&tokens.iter().map(cl_ln_from_token).collect::<Vec<ClLn>>())
// }
//
// pub fn cl_ln_from_many_cl_ln(cl_ln: &[ClLn]) -> ClLn {
//     let min_cl = cl_ln.iter().map(|t| t.1).min().unwrap();
//     let max_cl = cl_ln.iter().map(|t| t.3).max().unwrap();
//     let min_ln = cl_ln.iter().map(|t| t.0).min().unwrap();
//     let max_ln = cl_ln.iter().map(|t| t.2).max().unwrap();
//
//     (min_ln, min_cl, max_ln, max_cl)
// }

pub mod expression {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    node_macro!(
        pub enum Literal {
            Integer { value: i64 },
            Float { value: f64 },
            String { value: String },
            Boolean { value: bool },
        }
    );

    node_macro!(
        pub enum SingleDataUnit {
            Array {
                values: Vec<All>,
            },
            Literal {
                literal: Literal,
            },
            StructInit {
                namespace: Vec<String>,
                identifier: String,
                values: HashMap<String, All>,
            },
            Identifier {
                namespace: Vec<String>,
                identifier: String,
            },
            FunctionCall {
                function: Box<All>,
                arguments: Vec<All>,
            },
            ArrayAccess {
                array: Box<All>,
                index: Box<All>,
            },
            PropertyAccess {
                object: Box<All>,
                property: String,
            },
        }
    );

    node_macro!(
        pub enum Arithmetic {
            Neg { value: Box<All> },
            Add { left: Box<All>, right: Box<All> },
            Sub { left: Box<All>, right: Box<All> },
            Mul { left: Box<All>, right: Box<All> },
            Div { left: Box<All>, right: Box<All> },
            Mod { left: Box<All>, right: Box<All> },
            Pow { left: Box<All>, right: Box<All> },
        }
    );

    node_macro!(
        pub enum BooleanLogic {
            Or { left: Box<All>, right: Box<All> },
            And { left: Box<All>, right: Box<All> },

            Not { value: Box<All> },
        }
    );

    node_macro!(
        pub enum Cmp {
            Equal { left: Box<All>, right: Box<All> },
            NotEqual { left: Box<All>, right: Box<All> },
            LessThan { left: Box<All>, right: Box<All> },
            LessThanOrEqual { left: Box<All>, right: Box<All> },
            GreaterThan { left: Box<All>, right: Box<All> },
            GreaterThanOrEqual { left: Box<All>, right: Box<All> },
        }
    );

    node_macro!(
        pub enum All {
            SingleDataUnit { value: SingleDataUnit },
            Arithmetic { value: Arithmetic },
            BooleanLogic { value: BooleanLogic },
            Cmp { value: Cmp },
        }
    );
}
