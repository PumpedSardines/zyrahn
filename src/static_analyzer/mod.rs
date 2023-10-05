use std::collections::HashMap;

use crate::*;

mod expression;

pub enum Type {
    Integer,
    Float,
    Bool,
    String,
}

struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: HashMap<String, Type>,
}

impl<'a> Scope<'a> {
    fn new(parent: Option<&Scope<'a>>) -> Scope<'a> {
        Scope {
            parent,
            variables: HashMap::new(),
        }
    }

    fn get(&self, name: &str) -> Option<&Type> {
        self.variables
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.get(name)))
    }

    fn has(&self, name: &str) -> bool {
        self.variables.contains_key(name)
            || self
                .parent
                .as_ref()
                .map_or(false, |parent| parent.has(name))
    }

    fn set(&mut self, name: &str, value: Type) {
        self.variables.insert(name.to_string(), value);
    }
}

pub fn check(
    tree: &ast::node::expression::All,
) -> Result<(), error::Error<error::StaticAnalyzerErrorType>> {
    expression::eval_type(tree)
}
