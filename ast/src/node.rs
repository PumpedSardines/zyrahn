macro_rules! node_macro {
    (pub enum $x:ident { $($i:ident { $($k:ident : $v:ty$(,)?)+ },)+ }) => {
        #[derive(Clone, Serialize, Deserialize, Debug)]
        pub enum $x {
            $($i {
                $($k: $v,)+
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
