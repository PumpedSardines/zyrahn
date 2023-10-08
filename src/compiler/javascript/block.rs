use super::*;

pub fn compile(
    ast: &Vec<parser::node::block::All<parser::node::expression::AllWithType>>,
) -> String {
    let mut code = String::new();

    for block in ast {
        match block {
            parser::node::block::All::Expression { value, .. } => {
                code.push_str(&expression::compile(&value));
            }
            parser::node::block::All::VariableDeclaration {
                identifier, value, ..
            } => {
                code.push_str(
                    format!(
                        "let __0{} = {{ value: {} }};",
                        identifier,
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
