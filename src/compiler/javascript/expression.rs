use super::*;
use parser::node::expression;

pub fn compile(ast: &expression::AllWithType) -> String {
    match ast {
        expression::All::Cmp { value, .. } => match value {
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
        expression::All::Arithmetic { value, .. } => match value {
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
        expression::All::BooleanLogic { value, .. } => match value {
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
        expression::All::SingleDataUnit { value, .. } => match value {
            expression::SingleDataUnit::Literal { literal, .. } => match literal {
                expression::Literal::Float { value, .. } => {
                    format!("{}", value)
                }
                expression::Literal::Integer { value, .. } => {
                    format!("{}", value)
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
                format!("{}__0{}.value", namespace.join("0"), identifier)
            }
            expression::SingleDataUnit::FunctionCall {
                function,
                arguments,
                ..
            } => {
                let name = match function.as_ref() {
                    expression::All::SingleDataUnit { value, .. } => match value {
                        expression::SingleDataUnit::Identifier {
                            namespace,
                            identifier,
                            ..
                        } => {
                            format!("{}__0{}", namespace.join("0"), identifier)
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
                        .map(|x| compile(x))
                        .map(|s| format!("JSON.parse(JSON.stringify({}))", s))
                        .collect::<Vec<String>>()
                        .join(", "),
                )
            }
            expression::SingleDataUnit::ArrayInit { .. } => todo!(),
            expression::SingleDataUnit::ArrayAccess { .. } => todo!(),
            expression::SingleDataUnit::PropertyAccess { .. } => todo!(),
            expression::SingleDataUnit::StructInit { .. } => todo!(),
        },
        _ => todo!(),
    }
}
