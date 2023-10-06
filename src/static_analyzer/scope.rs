use std::collections::HashMap;

use super::Type;

pub struct Scope<'a> {
    // This way of doing it might be quite inefficient. Since only variables are allowed in lower
    // scopes. So get_function for example will need to be called recursively until it reaches the
    // top scope.
    parent: Option<&'a Scope<'a>>,
    variables: HashMap<String, Type>,
    // Why is the look up for functions a double vector?
    // First vector is function overloads, second vector is the types of the arguments.
    functions: HashMap<String, Vec<(Vec<Type>, Type)>>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<&'a Scope<'a>>) -> Scope<'a> {
        Scope {
            parent,
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_function<'b>(&self, ns: &Vec<String>, name: &str, args: &Vec<Type>) -> Option<Type> {
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

    pub fn set_function(&mut self, name: &str, args: Vec<Type>, ret_type: Type) {
        self.functions
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push((args, ret_type));
    }

    pub fn has_function(&self, ns: &Vec<String>, name: &str) -> bool {
        let name_with_ns = Scope::combine_ns_name(ns, name);

        self.functions.contains_key(&name_with_ns)
            || self
                .parent
                .as_ref()
                .map_or(false, |parent| parent.has_function(ns, name))
    }

    pub fn get_variable(&self, ns: &Vec<String>, name: &str) -> Option<&Type> {
        let name_with_ns = Scope::combine_ns_name(ns, name);

        self.variables.get(&name_with_ns).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_variable(ns, name))
        })
    }

    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
            || self
                .parent
                .as_ref()
                .map_or(false, |parent| parent.has_variable(name))
    }

    pub fn set_variable(&mut self, name: &str, value: Type) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn combine_ns_name(ns: &Vec<String>, name: &str) -> String {
        if ns.len() != 0 {
            format!("{}::{}", ns.join("::"), name)
        } else {
            name.to_string()
        }
    }
}
