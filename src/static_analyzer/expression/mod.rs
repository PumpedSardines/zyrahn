use crate::*;
use cl_ln::ClLn;
use parser::node::*;

/// Calculates what type both expressions have and return errors if they don't match
fn calc_type(
    left: &Node<expression::All>,
    right: &Node<expression::All>,
    scope: &static_analyzer::Scope,
) -> Result<common::Type, Vec<error::Error<error::StaticAnalyzerErrorType>>> {
    let left = evaluate(left, scope);
    let right = evaluate(right, scope);

    if left.is_err() && right.is_err() {
        return Err(left
            .unwrap_err()
            .into_iter()
            .chain(right.unwrap_err())
            .collect());
    }

    if left.is_err() || right.is_err() {
        let (not_error_type, prev_err) = if left.is_err() {
            (right.clone().unwrap().node.ty(), left.clone().unwrap_err())
        } else {
            (left.clone().unwrap().node.ty(), right.clone().unwrap_err())
        };

        let err = vec![error::Error::from_cl_ln(
            error::StaticAnalyzerErrorType::OperationNotSupported(
                lexer::TokenType::Add,
                not_error_type,
            ),
            &left.unwrap_err().first().unwrap().cl_ln(),
        )];
        return Err(prev_err.into_iter().chain(err).collect());
    }

    let left = left?;
    let right = right?;

    let left_ty = left.node.ty();
    let right_ty = right.node.ty();

    if left_ty == right_ty {
        Ok(left_ty)
    } else {
        Err(vec![error::Error::from_cl_ln(
            error::StaticAnalyzerErrorType::TypeMismatchOp(
                lexer::TokenType::Add,
                left_ty,
                right_ty,
            ),
            &left.cl_ln(),
        )])
    }
}

pub fn evaluate(
    node: &Node<expression::All>,
    scope: &static_analyzer::Scope,
) -> Result<Node<expression::AllWithType>, Vec<error::Error<error::StaticAnalyzerErrorType>>> {
    macro_rules! with_type {
        ($scope:ident::$name:ident, $l:expr, $r:expr, $ty:expr) => {{
            Node::from_cl_ln(
                expression::AllWithType::$scope {
                    value: expression::$scope::$name {
                        left: Box::new($l),
                        right: Box::new($r),
                    },
                    ty: $ty,
                },
                node,
            )
        }};
    }

    macro_rules! check_type {
        ($opt:tt, $ty:expr, $allowed_ty:expr, $if_allowed:block) => {
            if $allowed_ty.contains(&$ty) {
                $if_allowed
            } else {
                Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::OperationNotSupported(
                        lexer::TokenType::$opt,
                        $ty,
                    ),
                    node,
                )])
            }
        };
    }

    match node.node {
        expression::All::CompilerCustomCodePreDefined { .. } => {
            return Err(vec![error::Error::from_cl_ln(
                error::StaticAnalyzerErrorType::CompilerCustomCodePreDefined,
                node,
            )])
        }
        expression::All::SingleDataUnit { value, .. } => match value {
            expression::SingleDataUnit::Literal { literal, .. } => match literal {
                expression::Literal::Integer { value } => Ok(Node::from_cl_ln(
                    expression::AllWithType::SingleDataUnit {
                        value: expression::SingleDataUnit::Literal {
                            literal: expression::Literal::Integer { value },
                        },
                        ty: common::Type::Integer,
                    },
                    node,
                )),
                expression::Literal::Float { value } => Ok(Node::from_cl_ln(
                    expression::AllWithType::SingleDataUnit {
                        value: expression::SingleDataUnit::Literal {
                            literal: expression::Literal::Float { value },
                        },
                        ty: common::Type::Float,
                    },
                    node,
                )),
                expression::Literal::String { value } => Ok(Node::from_cl_ln(
                    expression::AllWithType::SingleDataUnit {
                        value: expression::SingleDataUnit::Literal {
                            literal: expression::Literal::String { value },
                        },
                        ty: common::Type::String,
                    },
                    node,
                )),
                expression::Literal::Boolean { value } => Ok(Node::from_cl_ln(
                    expression::AllWithType::SingleDataUnit {
                        value: expression::SingleDataUnit::Literal {
                            literal: expression::Literal::Boolean { value },
                        },
                        ty: common::Type::Boolean,
                    },
                    node,
                )),
            },
            expression::SingleDataUnit::FunctionCall {
                function,
                arguments,
                ..
            } => match function.as_ref() {
                Node {
                    node: expression::All::SingleDataUnit { value, .. },
                    ..
                } => match value {
                    expression::SingleDataUnit::Identifier {
                        namespace,
                        identifier,
                    } => {
                        let args = arguments
                            .iter()
                            .map(|(is_out, arg)| {
                                let arg = evaluate(arg, scope);

                                if arg.is_err() {
                                    return Err(arg.unwrap_err());
                                } else {
                                    Ok((is_out.clone(), arg.unwrap()))
                                }
                            })
                            .collect::<Result<Vec<(bool, Node<expression::AllWithType>)>, _>>()?;

                        let args_types = args.iter().map(|(_, arg)| arg.node.ty()).collect();

                        if let Some(ret_type) =
                            scope.get_function(namespace, identifier, &args_types)
                        {
                            Ok(Node::from_cl_ln(
                                expression::AllWithType::SingleDataUnit {
                                    value: expression::SingleDataUnit::FunctionCall {
                                        function: Box::new(Node::from_cl_ln(
                                            expression::AllWithType::SingleDataUnit {
                                                value: expression::SingleDataUnit::Identifier {
                                                    namespace: namespace.clone(),
                                                    identifier: identifier.clone(),
                                                },
                                                ty: common::Type::Never,
                                            },
                                            node,
                                        )),
                                        arguments: args,
                                    },
                                    ty: ret_type.clone(),
                                },
                                node,
                            ))
                        } else {
                            if scope.has_function(namespace, identifier) {
                                return Err(vec![error::Error::from_cl_ln(
                                    error::StaticAnalyzerErrorType::FunctionArgumentMismatch(
                                        identifier.clone(),
                                        namespace.clone(),
                                        args_types,
                                    ),
                                    node,
                                )]);
                            }

                            Err(vec![error::Error::from_cl_ln(
                                error::StaticAnalyzerErrorType::FunctionNotDefined(
                                    identifier.clone(),
                                    namespace.clone(),
                                ),
                                node,
                            )])
                        }
                    }
                    _ => {
                        return Err(vec![error::Error::from_cl_ln(
                            error::StaticAnalyzerErrorType::CannotCallNonFunction,
                            node,
                        )])
                    }
                },
                _ => {
                    return Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::CannotCallNonFunction,
                        node,
                    )])
                }
            },
            expression::SingleDataUnit::Identifier {
                identifier,
                namespace,
                ..
            } => {
                if let Some(value) = scope.get_variable(&namespace, &identifier) {
                    Ok(Node::from_cl_ln(
                        expression::AllWithType::SingleDataUnit {
                            value: expression::SingleDataUnit::Identifier {
                                identifier: identifier.clone(),
                                namespace: namespace.clone(),
                            },
                            ty: value.clone(),
                        },
                        node,
                    ))
                } else {
                    Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::VariableNotDefined(
                            identifier.clone(),
                            namespace.clone(),
                        ),
                        node,
                    )])
                }
            }
            expression::SingleDataUnit::ArrayInit { .. } => {
                return Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::FeatureNotImplemented("Array init".to_string()),
                    node,
                )])
            }
            expression::SingleDataUnit::StructInit { .. } => {
                return Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::FeatureNotImplemented(
                        "Struct init".to_string(),
                    ),
                    node,
                )])
            }
            expression::SingleDataUnit::ArrayAccess { .. } => {
                return Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::FeatureNotImplemented(
                        "Array access".to_string(),
                    ),
                    node,
                )])
            }
            expression::SingleDataUnit::PropertyAccess { .. } => {
                return Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::FeatureNotImplemented(
                        "Property access".to_string(),
                    ),
                    node,
                )])
            }
        },
        expression::All::Cmp { value, .. } => match value {
            expression::Cmp::Equal { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(
                    Equal,
                    ty,
                    vec![
                        common::Type::Integer,
                        common::Type::Float,
                        common::Type::String,
                        common::Type::Boolean
                    ],
                    {
                        let left = evaluate(&left, scope)?;
                        let right = evaluate(&right, scope)?;

                        Ok(with_type!(Cmp::Equal, left, right, ty))
                    }
                )
            }
            expression::Cmp::NotEqual { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(
                    NotEqual,
                    ty,
                    vec![
                        common::Type::Integer,
                        common::Type::Float,
                        common::Type::String,
                        common::Type::Boolean
                    ],
                    {
                        let left = evaluate(&left, scope)?;
                        let right = evaluate(&right, scope)?;

                        Ok(with_type!(Cmp::NotEqual, left, right, ty))
                    }
                )
            }
            expression::Cmp::LessThan { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(
                    LessThan,
                    ty,
                    vec![common::Type::Integer, common::Type::Float],
                    {
                        let left = evaluate(&left, scope)?;
                        let right = evaluate(&right, scope)?;

                        Ok(with_type!(Cmp::LessThan, left, right, ty))
                    }
                )
            }
            expression::Cmp::LessThanOrEqual { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(
                    LessThanOrEqual,
                    ty,
                    vec![common::Type::Integer, common::Type::Float],
                    {
                        let left = evaluate(&left, scope)?;
                        let right = evaluate(&right, scope)?;

                        Ok(with_type!(Cmp::LessThanOrEqual, left, right, ty))
                    }
                )
            }
            expression::Cmp::GreaterThan { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(
                    GreaterThan,
                    ty,
                    vec![common::Type::Integer, common::Type::Float],
                    {
                        let left = evaluate(&left, scope)?;
                        let right = evaluate(&right, scope)?;

                        Ok(with_type!(Cmp::GreaterThan, left, right, ty))
                    }
                )
            }
            expression::Cmp::GreaterThanOrEqual { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(
                    GreaterThanOrEqual,
                    ty,
                    vec![common::Type::Integer, common::Type::Float],
                    {
                        let left = evaluate(&left, scope)?;
                        let right = evaluate(&right, scope)?;

                        Ok(with_type!(Cmp::GreaterThanOrEqual, left, right, ty))
                    }
                )
            }
        },
        expression::All::BooleanLogic { value, .. } => match value {
            expression::BooleanLogic::And { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(And, ty, vec![common::Type::Boolean], {
                    let left = evaluate(&left, scope)?;
                    let right = evaluate(&right, scope)?;

                    Ok(with_type!(BooleanLogic::And, left, right, ty))
                })
            }
            expression::BooleanLogic::Or { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(Or, ty, vec![common::Type::Boolean], {
                    let left = evaluate(&left, scope)?;
                    let right = evaluate(&right, scope)?;

                    Ok(with_type!(BooleanLogic::Or, left, right, ty))
                })
            }
            expression::BooleanLogic::Not { value, .. } => {
                let value = evaluate(&value, scope)?;
                let ty = value.node.ty();

                if ty == common::Type::Boolean {
                    Ok(Node::from_cl_ln(
                        expression::AllWithType::BooleanLogic {
                            value: expression::BooleanLogic::Not {
                                value: Box::new(value),
                            },
                            ty,
                        },
                        node,
                    ))
                } else {
                    Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::OperationNotSupportedNot(ty),
                        node,
                    )])
                }
            }
        },
        expression::All::Arithmetic { value, .. } => match value {
            expression::Arithmetic::Add { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(
                    Add,
                    ty,
                    vec![
                        common::Type::Integer,
                        common::Type::Float,
                        common::Type::String
                    ],
                    {
                        let left = evaluate(&left, scope)?;
                        let right = evaluate(&right, scope)?;

                        Ok(with_type!(Arithmetic::Add, left, right, ty))
                    }
                )
            }
            expression::Arithmetic::Sub { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(Sub, ty, vec![common::Type::Integer, common::Type::Float], {
                    let left = evaluate(&left, scope)?;
                    let right = evaluate(&right, scope)?;

                    Ok(with_type!(Arithmetic::Sub, left, right, ty))
                })
            }
            expression::Arithmetic::Mul { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(Mul, ty, vec![common::Type::Integer, common::Type::Float], {
                    let left = evaluate(&left, scope)?;
                    let right = evaluate(&right, scope)?;

                    Ok(with_type!(Arithmetic::Mul, left, right, ty))
                })
            }
            expression::Arithmetic::Div { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(Div, ty, vec![common::Type::Integer, common::Type::Float], {
                    let left = evaluate(&left, scope)?;
                    let right = evaluate(&right, scope)?;

                    Ok(with_type!(Arithmetic::Div, left, right, ty))
                })
            }
            expression::Arithmetic::Mod { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(Mod, ty, vec![common::Type::Integer, common::Type::Float], {
                    let left = evaluate(&left, scope)?;
                    let right = evaluate(&right, scope)?;

                    Ok(with_type!(Arithmetic::Mod, left, right, ty))
                })
            }
            expression::Arithmetic::Pow { left, right, .. } => {
                let ty = calc_type(&left, &right, scope)?;

                check_type!(Pow, ty, vec![common::Type::Integer, common::Type::Float], {
                    let left = evaluate(&left, scope)?;
                    let right = evaluate(&right, scope)?;

                    Ok(with_type!(Arithmetic::Pow, left, right, ty))
                })
            }
            expression::Arithmetic::Neg { value, .. } => {
                let value = evaluate(&value, scope)?;
                let ty = value.node.ty();

                if ty == common::Type::Integer || ty == common::Type::Float {
                    Ok(Node::from_cl_ln(
                        expression::AllWithType::Arithmetic {
                            value: expression::Arithmetic::Neg {
                                value: Box::new(value),
                            },
                            ty,
                        },
                        node,
                    ))
                } else {
                    Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::OperationNotSupportedNeg(ty),
                        node,
                    )])
                }
            }
        },
    }
}
