use crate::{parser::node::Node, *};

mod block;
mod expression;

pub fn compile(
    ast: &Vec<Node<parser::node::block::All<Node<parser::node::expression::AllWithType>>>>,
) -> String {
    let code = block::compile(ast);

    return code;
}
