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

use crate::{parser::node::Node, *};

mod expression;
mod scope;
use cl_ln::ClLn;
pub use scope::Scope;
mod block;

pub fn evaluate(
    tree: &Vec<Node<parser::node::block::All<Node<parser::node::expression::All>>>>,
) -> Result<
    Vec<Node<parser::node::block::All<Node<parser::node::expression::AllWithType>>>>,
    Vec<error::Error<error::StaticAnalyzerErrorType>>,
> {
    let mut scope = Scope::new(None);
    scope.set_function(
        "std::test",
        vec![
            (false, common::Type::Integer),
            (true, common::Type::Integer),
        ],
        common::Type::Boolean,
    );

    let block_eval = block::check(tree, &mut scope);

    if let Err(errs) = block_eval {
        return Err(errs);
    }

    Ok(block_eval.unwrap())
}
