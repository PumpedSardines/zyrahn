use crate::*;

macro_rules! node_macro {
    (pub enum $x:ident { $($i:ident { $($k:ident : $v:ty$(,)?)* },)+ }) => {
        #[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
        pub enum $x {
            $($i {
                $($k: $v,)*
                // ln_start, cl_start, ln_end, cl_end
                cl_ln: (usize, usize, usize, usize)
            },)+
        }

        impl cl_ln::ClLn for $x {
            fn cl_start(&self) -> usize {
                match self {
                    $($x::$i { cl_ln, .. } => cl_ln.1 ),+
                }
            }

            fn cl_end(&self) -> usize {
                match self {
                    $($x::$i { cl_ln, .. } => cl_ln.3 ),+
                }
            }

            fn ln_start(&self) -> usize {
                match self {
                    $($x::$i { cl_ln, .. } => cl_ln.0 ),+
                }
            }

            fn ln_end(&self) -> usize {
                match self {
                    $($x::$i { cl_ln, .. } => cl_ln.2 ),+
                }
            }
        }
    };
}

pub mod expression {
    use super::*;
    use std::collections::HashMap;

    node_macro! {
        pub enum Literal {
            Integer { value: i64 },
            Float { value: f64 },
            String { value: String },
            Boolean { value: bool },
        }
    }

    node_macro! {
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
    }

    node_macro! {
        pub enum Arithmetic {
            Neg { value: Box<All> },
            Add { left: Box<All>, right: Box<All> },
            Sub { left: Box<All>, right: Box<All> },
            Mul { left: Box<All>, right: Box<All> },
            Div { left: Box<All>, right: Box<All> },
            Mod { left: Box<All>, right: Box<All> },
            Pow { left: Box<All>, right: Box<All> },
        }
    }

    node_macro! {
        pub enum BooleanLogic {
            Or { left: Box<All>, right: Box<All> },
            And { left: Box<All>, right: Box<All> },

            Not { value: Box<All> },
        }
    }

    node_macro! {
        pub enum Cmp {
            Equal { left: Box<All>, right: Box<All> },
            NotEqual { left: Box<All>, right: Box<All> },
            LessThan { left: Box<All>, right: Box<All> },
            LessThanOrEqual { left: Box<All>, right: Box<All> },
            GreaterThan { left: Box<All>, right: Box<All> },
            GreaterThanOrEqual { left: Box<All>, right: Box<All> },
        }
    }

    node_macro! {
        pub enum All {
            SingleDataUnit { value: SingleDataUnit },
            Arithmetic { value: Arithmetic },
            BooleanLogic { value: BooleanLogic },
            Cmp { value: Cmp },
        }
    }
}

mod block {
    use super::*;

    node_macro! {
        pub enum All {
            Expression { value: expression::All },
            If { cond: expression::All, then_body: Vec<All> },
            IfElse { cond: expression::All, then_body: Vec<All>, else_body: Vec<All> },
            While { cond: expression::All, body: Vec<All> },
            VariableDeclaration { identifier: String, r#type: Type, value: expression::All },
            VariableAssignment { identifier: String, value: expression::All },
            Return { value: Option<expression::All> },
            Break { },
            Continue { },
        }
    }

    node_macro! {
        pub enum Type {
            Integer { },
            Float { },
            String { },
            Boolean { },
            // This allows for nested arrays, which are not supported by the language
            // but are supported by the parser. So we'll check for this is the type checker
            Array { inner: Box<Type> },
            Struct { namespace: Vec<String>, identifier: String },
        }
    }
}
