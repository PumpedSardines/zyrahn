use crate::*;

mod block;
mod expression;

pub fn compile(
    ast: &Vec<parser::node::block::All<parser::node::expression::AllWithType>>,
) -> String {
    let code = block::compile(ast);

    return code;
}
