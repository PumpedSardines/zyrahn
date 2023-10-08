use super::*;

pub fn compile(
    nodes: &Vec<Node<parser::node::block::All<Node<parser::node::expression::AllWithType>>>>,
) -> String {
    let mut code = String::new();

    for node in nodes {
        match &node.node {
            parser::node::block::All::Expression { value, .. } => {
                code.push_str(&expression::compile(&value));
            }
            parser::node::block::All::VariableDeclaration {
                identifier, value, ..
            } => {
                code.push_str(
                    format!(
                        "let {} = {{ value: {} }};",
                        get_var_name(vec![], &identifier),
                        expression::compile(&value)
                    )
                    .as_str(),
                );
            }
            _ => todo!(),
        };
    }

    return code;
}
