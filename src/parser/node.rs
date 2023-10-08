use crate::*;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub node: T,
    cl_ln: (usize, usize, usize, usize),
}

impl<T> Node<T> {
    pub fn from_cl_ln<C: cl_ln::ClLn + Sized>(node: T, cl_ln: &C) -> Self {
        Self {
            node,
            cl_ln: cl_ln.cl_ln(),
        }
    }
}

impl<T> cl_ln::ClLn for Node<T> {
    fn ln_start(&self) -> usize {
        self.cl_ln.0
    }

    fn ln_end(&self) -> usize {
        self.cl_ln.2
    }

    fn cl_start(&self) -> usize {
        self.cl_ln.1
    }

    fn cl_end(&self) -> usize {
        self.cl_ln.3
    }
}

pub mod expression {
    use super::*;
    use std::collections::HashMap;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Literal {
        Integer { value: i64 },
        Float { value: f64 },
        String { value: String },
        Boolean { value: bool },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum SingleDataUnit<T: Clone + Debug + PartialEq> {
        ArrayInit {
            values: Vec<T>,
        },
        Literal {
            literal: Literal,
        },
        StructInit {
            namespace: Vec<String>,
            identifier: String,
            values: HashMap<String, T>,
        },
        Identifier {
            namespace: Vec<String>,
            identifier: String,
        },
        FunctionCall {
            function: Box<T>,
            arguments: Vec<(bool, T)>,
        },
        ArrayAccess {
            array: Box<T>,
            index: Box<T>,
        },
        PropertyAccess {
            object: Box<T>,
            property: String,
        },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Arithmetic<T: Clone + Debug + PartialEq> {
        Neg { value: Box<T> },
        Add { left: Box<T>, right: Box<T> },
        Sub { left: Box<T>, right: Box<T> },
        Mul { left: Box<T>, right: Box<T> },
        Div { left: Box<T>, right: Box<T> },
        Mod { left: Box<T>, right: Box<T> },
        Pow { left: Box<T>, right: Box<T> },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum BooleanLogic<T: Clone + Debug + PartialEq> {
        Or { left: Box<T>, right: Box<T> },
        And { left: Box<T>, right: Box<T> },
        Not { value: Box<T> },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Cmp<T: Clone + Debug + PartialEq> {
        Equal { left: Box<T>, right: Box<T> },
        NotEqual { left: Box<T>, right: Box<T> },
        LessThan { left: Box<T>, right: Box<T> },
        LessThanOrEqual { left: Box<T>, right: Box<T> },
        GreaterThan { left: Box<T>, right: Box<T> },
        GreaterThanOrEqual { left: Box<T>, right: Box<T> },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum All {
        SingleDataUnit { value: SingleDataUnit<Node<All>> },
        CompilerCustomCodePreDefined { value: String },
        Arithmetic { value: Arithmetic<Node<All>> },
        BooleanLogic { value: BooleanLogic<Node<All>> },
        Cmp { value: Cmp<Node<All>> },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum AllWithType {
        SingleDataUnit {
            value: SingleDataUnit<Node<AllWithType>>,
            ty: common::Type,
        },
        CompilerCustomCodePreDefined {
            value: String,
            ty: common::Type,
        },
        Arithmetic {
            value: Arithmetic<Node<AllWithType>>,
            ty: common::Type,
        },
        BooleanLogic {
            value: BooleanLogic<Node<AllWithType>>,
            ty: common::Type,
        },
        Cmp {
            value: Cmp<Node<AllWithType>>,
            ty: common::Type,
        },
    }

    impl AllWithType {
        pub fn ty(&self) -> common::Type {
            match self {
                AllWithType::SingleDataUnit { ty, .. } => ty.clone(),
                AllWithType::CompilerCustomCodePreDefined { ty, .. } => ty.clone(),
                AllWithType::Arithmetic { ty, .. } => ty.clone(),
                AllWithType::BooleanLogic { ty, .. } => ty.clone(),
                AllWithType::Cmp { ty, .. } => ty.clone(),
            }
        }
    }
}

pub mod block {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    pub enum All<T: Sized> {
        Expression {
            value: T,
        },
        If {
            cond: T,
            then_body: Vec<All<T>>,
        },
        IfElse {
            cond: T,
            then_body: Vec<All<T>>,
            else_body: Vec<All<T>>,
        },
        While {
            cond: T,
            body: Vec<All<T>>,
        },
        VariableDeclaration {
            identifier: String,
            ty: common::Type,
            value: T,
        },
        VariableAssignment {
            identifier: String,
            value: T,
        },
        Return {
            value: Option<T>,
        },
        Break {},
        Continue {},
    }
}
