use super::*;

pub fn check(
    tree: &Vec<Node<parser::node::block::All<Node<parser::node::expression::All>>>>,
    scope: &mut static_analyzer::Scope,
) -> Result<
    Vec<Node<parser::node::block::All<Node<parser::node::expression::AllWithType>>>>,
    Vec<error::Error<error::StaticAnalyzerErrorType>>,
> {
    let mut ret_blocks = vec![];
    let mut errors = vec![];

    for node in tree {
        match &node.node {
            parser::node::block::All::VariableDeclaration {
                identifier,
                ty,
                value,
            } => {
                scope.set_variable(&identifier, ty.clone());

                let exp = expression::evaluate(&value, &scope);

                if let Err(errs) = exp {
                    errors.extend(errs);
                    continue;
                }

                let exp_ty = exp.unwrap();

                if exp_ty.node.ty() != *ty {
                    errors.push(error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::TypeMismatchAssign(
                            ty.clone(),
                            exp_ty.node.ty(),
                        ),
                        &exp_ty,
                    ));
                }

                ret_blocks.push(Node::from_cl_ln(
                    parser::node::block::All::VariableDeclaration {
                        identifier: identifier.clone(),
                        ty: ty.clone(),
                        value: exp_ty,
                    },
                    node,
                ));
            }
            parser::node::block::All::Expression { value, .. } => {
                let exp = expression::evaluate(&value, &scope);

                if let Err(errs) = exp {
                    errors.extend(errs);
                } else {
                    ret_blocks.push(Node::from_cl_ln(
                        parser::node::block::All::Expression {
                            value: exp.unwrap(),
                        },
                        &value.cl_ln(),
                    ));
                }
            }
            _ => {
                return Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::FeatureNotImplemented("".to_string()),
                    &node.cl_ln(),
                )]);
            }
        }
    }

    if errors.len() > 0 {
        return Err(errors);
    }

    Ok(ret_blocks)
}
