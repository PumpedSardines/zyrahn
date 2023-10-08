use crate::{parser::node::Node, *};

mod block;
mod expression;
mod std;

fn get_var_name(namespace: &Vec<String>, identifier: &String) -> String {
    format!("v{}__0{}", namespace.join("0"), identifier)
}

fn get_func_name(
    namespace: &Vec<String>,
    identifier: &String,
    args: &Vec<(bool, common::Type)>,
) -> String {
    let arg_types = args
        .clone()
        .into_iter()
        .map(|(is_out, ty)| format!("{}{}", if is_out { "out_" } else { "" }, ty))
        .collect::<Vec<_>>();

    format!(
        "f{}__0{}__0{}",
        namespace.join("0"),
        arg_types.join("0"),
        identifier
    )
}

pub fn compile(
    ast: &Vec<Node<parser::node::block::All<Node<parser::node::expression::AllWithType>>>>,
) -> String {
    let code = block::compile(ast);

    return code;
}
