use super::*;
use parser::node::expression;

pub fn compile(node: &Node<expression::AllWithType>) -> String {
    let node = &node.node;
    match node {
        expression::AllWithType::Cmp { value, .. } => match value {
            expression::Cmp::Equal { left, right, .. } => {
                format!("({}) == ({})", compile(left), compile(right))
            }
            expression::Cmp::NotEqual { left, right, .. } => {
                format!("({}) != ({})", compile(left), compile(right))
            }
            expression::Cmp::LessThan { left, right, .. } => {
                format!("({}) < ({})", compile(left), compile(right))
            }
            expression::Cmp::LessThanOrEqual { left, right, .. } => {
                format!("({}) <= ({})", compile(left), compile(right))
            }
            expression::Cmp::GreaterThan { left, right, .. } => {
                format!("({}) > {}", compile(left), compile(right))
            }
            expression::Cmp::GreaterThanOrEqual { left, right, .. } => {
                format!("({}) >= ({})", compile(left), compile(right))
            }
        },
        expression::AllWithType::Arithmetic { value, .. } => match value {
            expression::Arithmetic::Add { left, right, .. } => {
                format!("({}) + ({})", compile(left), compile(right))
            }
            expression::Arithmetic::Sub { left, right, .. } => {
                format!("({}) - ({})", compile(left), compile(right))
            }
            expression::Arithmetic::Mul { left, right, .. } => {
                format!("({}) * ({})", compile(left), compile(right))
            }
            expression::Arithmetic::Div { left, right, .. } => {
                format!("({}) / ({})", compile(left), compile(right))
            }
            expression::Arithmetic::Mod { left, right, .. } => {
                format!("({}) % ({})", compile(left), compile(right))
            }
            expression::Arithmetic::Pow { left, right, .. } => {
                format!("({}) ** ({})", compile(left), compile(right))
            }
            expression::Arithmetic::Neg { value, .. } => {
                format!("-({})", compile(value))
            }
        },
        expression::AllWithType::BooleanLogic { value, .. } => match value {
            expression::BooleanLogic::Or { left, right, .. } => {
                format!("({}) || ({})", compile(left), compile(right))
            }
            expression::BooleanLogic::And { left, right, .. } => {
                format!("({}) && ({})", compile(left), compile(right))
            }
            expression::BooleanLogic::Not { value, .. } => {
                format!("!({})", compile(value))
            }
        },
        expression::AllWithType::SingleDataUnit { value, .. } => {
            match value {
                expression::SingleDataUnit::Literal { literal, .. } => match literal {
                    expression::Literal::Float { value, .. } => {
                        format!("{}", value)
                    }
                    expression::Literal::Integer { value, .. } => {
                        format!("BigInt({})", value)
                    }
                    expression::Literal::String { value, .. } => {
                        format!("\"{}\"", value)
                    }
                    expression::Literal::Boolean { value, .. } => {
                        format!("{}", value)
                    }
                },
                expression::SingleDataUnit::Identifier {
                    namespace,
                    identifier,
                    ..
                } => {
                    // Since a variable in zyrahn can't have numbers in them we can safetly combine
                    // namespace like this
                    format!("v{}__0{}.value", namespace.join("0"), identifier)
                }
                expression::SingleDataUnit::FunctionCall {
                    function,
                    arguments,
                    ..
                } => {
                    let name = match function.as_ref() {
                        Node {
                            node: expression::AllWithType::SingleDataUnit { value, .. },
                            ..
                        } => match value {
                            expression::SingleDataUnit::Identifier {
                                namespace,
                                identifier,
                                ..
                            } => {
                                let arg_types = arguments
                                    .clone()
                                    .into_iter()
                                    .map(|(is_out, x)| {
                                        format!(
                                            "{}{}",
                                            if is_out { "out_" } else { "" },
                                            x.node.ty()
                                        )
                                    })
                                    .collect::<Vec<_>>();

                                format!(
                                    "f{}__0{}__0{}",
                                    namespace.join("0"),
                                    arg_types.join("0"),
                                    identifier
                                )
                            }
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };

                    format!(
                        "{}({})",
                        name,
                        arguments
                            .iter()
                            .map(|(is_out, x)| {
                                if *is_out {
                                    match x {
                                        Node {
                                            node:
                                                expression::AllWithType::SingleDataUnit{ 
                                                    value: expression::SingleDataUnit::Identifier {
                                                        namespace,
                                                        identifier,
                                                        ..
                                                    },
                                                ..
                                                },
                                            ..
                                        } => {
                                            format!("v{}__0{}", namespace.join("0"), identifier)
                                        }
                                        _ => {
                                            panic!("Static analyzer has given a tree that is not valid")
                                        }
                                    }
                                } else {
                                    format!("{{value:JSON.parse(JSON.stringify({}))}}", compile(x))
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", "),
                    )
                }
                expression::SingleDataUnit::ArrayInit { .. } => todo!(),
                expression::SingleDataUnit::ArrayAccess { .. } => todo!(),
                expression::SingleDataUnit::PropertyAccess { .. } => todo!(),
                expression::SingleDataUnit::StructInit { .. } => todo!(),
            }
        }
        _ => todo!(),
    }
}
