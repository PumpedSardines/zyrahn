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
mod scope;
mod r#type;
pub use r#type::Type;
pub use scope::Scope;

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
