//! Syntax tree static analyzer.
//!
//! This module is responsible for checking the syntax tree for errors that can be caught at compile
//! time. This includes type checking, variable/function usage, etc.
//!
//! # Examples
//! ```
//! 3 + 4 // Ok
//! 3 + "4" // Error
//! ```

use crate::*;

mod expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    String,
    Array(Box<Type>),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Type::*;

        match self {
            Integer => write!(f, "Integer"),
            Float => write!(f, "Float"),
            Boolean => write!(f, "Boolean"),
            String => write!(f, "String"),
            Array(t) => write!(f, "{}[]", t),
            Void => write!(f, "Void"),
        }
    }
}

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: HashMap<String, Type>,
    // Why is the look up for functions a double vector?
    // First vector is function overloads, second vector is the types of the arguments.
    functions: HashMap<String, Vec<(Vec<Type>, Type)>>,
}

impl<'a> Scope<'a> {
    fn new(parent: Option<&'a Scope<'a>>) -> Scope<'a> {
        Scope {
            parent,
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn get_function<'b>(&self, ns: &Vec<String>, name: &str, args: &Vec<Type>) -> Option<Type> {
        let name_with_ns = Scope::combine_ns_name(ns, name);

        self.functions
            .get(&name_with_ns)
            .map_or(None, |overloads| {
                overloads
                    .iter()
                    .find(|(overload_args, _)| {
                        if args.len() == overload_args.len() {
                            args.iter().zip(overload_args.iter()).all(|(a, b)| a == b)
                        } else {
                            false
                        }
                    })
                    .map(|(_, ret_type)| ret_type)
                    .clone()
            })
            .cloned()
            .or_else(|| {
                self.parent
                    .as_ref()
                    .and_then(|parent| parent.get_function(ns, name, args))
                    .map_or(None, |ret_type| Some(ret_type.clone()))
            })
    }

    fn set_function(&mut self, name: &str, args: Vec<Type>, ret_type: Type) {
        self.functions
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push((args, ret_type));
    }

    fn has_function(&self, ns: &Vec<String>, name: &str) -> bool {
        let name_with_ns = Scope::combine_ns_name(ns, name);

        self.functions.contains_key(&name_with_ns)
            || self
                .parent
                .as_ref()
                .map_or(false, |parent| parent.has_function(ns, name))
    }

    fn get_variable(&self, ns: &Vec<String>, name: &str) -> Option<&Type> {
        let name_with_ns = Scope::combine_ns_name(ns, name);

        self.variables.get(&name_with_ns).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_variable(ns, name))
        })
    }

    fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
            || self
                .parent
                .as_ref()
                .map_or(false, |parent| parent.has_variable(name))
    }

    fn set_variable(&mut self, name: &str, value: Type) {
        self.variables.insert(name.to_string(), value);
    }

    fn combine_ns_name(ns: &Vec<String>, name: &str) -> String {
        if ns.len() != 0 {
            format!("{}::{}", ns.join("::"), name)
        } else {
            name.to_string()
        }
    }
}

pub fn check(
    tree: &ast::node::expression::All,
) -> Result<(), Vec<error::Error<error::StaticAnalyzerErrorType>>> {
    let mut scope = Scope::new(None);

    scope.set_variable("a", Type::Integer);
    scope.set_variable("b", Type::Integer);
    scope.set_function("add", vec![Type::Integer, Type::Integer], Type::Integer);
    scope.set_function("add", vec![Type::Float], Type::Integer);

    expression::eval_type(tree, &scope)?;

    Ok(())
}
