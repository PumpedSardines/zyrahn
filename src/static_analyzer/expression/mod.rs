use crate::*;
use ast::node::*;

pub fn eval_type(
    tree: &expression::All,
    scope: &static_analyzer::Scope,
) -> Result<static_analyzer::Type, Vec<error::Error<error::StaticAnalyzerErrorType>>> {
    // This should probably be a function lol (I'm lazy)
    macro_rules! type_check {
        (same ret, $operation:ident, $r:ident, $l:ident, $($supported_type:ident),+) => {{
            let left = eval_type(&$l, scope);
            let right = eval_type(&$r, scope);

            if left.is_err() && right.is_err() {
                return Err(left.unwrap_err().into_iter().chain(right.unwrap_err()).collect());
            }

            if left.is_err() || right.is_err() {
                let (not_error_type, prev_err) = if left.is_err() {
                    (right.clone().unwrap(), left.clone().unwrap_err())
                } else {
                    (left.clone().unwrap(), right.clone().unwrap_err())
                };

                if $(not_error_type != static_analyzer::Type::$supported_type &&)+ true {
                    let err = vec![error::Error::from_cl_ln(
                            error::StaticAnalyzerErrorType::OperationNotSupported(
                                lexer::TokenType::$operation,
                                not_error_type,
                            ),
                            tree,
                        )];
                    return Err(prev_err.into_iter().chain(err).collect());
                }
            }

            let left = left?;
            let right = right?;

            if left == right {
                if $(left != static_analyzer::Type::$supported_type &&)+ true {
                    return Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::OperationNotSupported(
                            lexer::TokenType::$operation,
                            left,
                        ),
                        tree,
                    )]);
                }

                Ok(left)
            } else {
                Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::TypeMismatch(
                        lexer::TokenType::$operation,
                        left,
                        right,
                    ),
                    tree,
                )])
            }
        }};
        ($rtype:ident, $operation:ident, $r:ident, $l:ident, $($supported_type:ident),+) => {{
            let left = eval_type(&$l, scope)?;
            let right = eval_type(&$r, scope)?;

            if left == right {
                if $(left != static_analyzer::Type::$supported_type &&)+ true {
                    return Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::OperationNotSupported(
                            lexer::TokenType::$operation,
                            left,
                        ),
                        tree,
                    )]);
                }

                Ok(static_analyzer::Type::$rtype)
            } else {
                Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::TypeMismatch(
                        lexer::TokenType::$operation,
                        left,
                        right,
                    ),
                    tree,
                )])
            }
        }};
    }

    match tree {
        expression::All::SingleDataUnit { value, .. } => match value {
            expression::SingleDataUnit::Literal { literal, .. } => match literal {
                expression::Literal::Integer { .. } => Ok(static_analyzer::Type::Integer),
                expression::Literal::Float { .. } => Ok(static_analyzer::Type::Float),
                expression::Literal::String { .. } => Ok(static_analyzer::Type::String),
                expression::Literal::Boolean { .. } => Ok(static_analyzer::Type::Boolean),
            },
            expression::SingleDataUnit::FunctionCall {
                function,
                arguments,
                ..
            } => match function.as_ref() {
                expression::All::SingleDataUnit { value, .. } => match value {
                    expression::SingleDataUnit::Identifier {
                        namespace,
                        identifier,
                        cl_ln,
                    } => {
                        let args = arguments
                            .iter()
                            .map(|arg| eval_type(arg, scope))
                            .collect::<Result<Vec<_>, _>>()?;

                        if let Some(ret_type) = scope.get_function(namespace, identifier, &args) {
                            Ok(ret_type.clone())
                        } else {
                            if scope.has_function(namespace, identifier) {
                                return Err(vec![error::Error::from_cl_ln(
                                    error::StaticAnalyzerErrorType::FunctionArgumentMismatch(
                                        identifier.clone(),
                                        namespace.clone(),
                                        args.clone(),
                                    ),
                                    cl_ln,
                                )]);
                            }

                            Err(vec![error::Error::from_cl_ln(
                                error::StaticAnalyzerErrorType::FunctionNotDefined(
                                    identifier.clone(),
                                    namespace.clone(),
                                ),
                                cl_ln,
                            )])
                        }
                    }
                    _ => {
                        return Err(vec![error::Error::from_cl_ln(
                            error::StaticAnalyzerErrorType::CannotCallNonFunction,
                            tree,
                        )])
                    }
                },
                _ => {
                    return Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::CannotCallNonFunction,
                        tree,
                    )])
                }
            },
            expression::SingleDataUnit::Identifier {
                identifier,
                namespace,
                ..
            } => {
                if let Some(value) = scope.get_variable(namespace, identifier) {
                    Ok(value.clone())
                } else {
                    Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::VariableNotDefined(
                            identifier.clone(),
                            namespace.clone(),
                        ),
                        tree,
                    )])
                }
            }
            expression::SingleDataUnit::ArrayInit { .. } => {
                return Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::FeatureNotImplemented("Array init".to_string()),
                    tree,
                )])
            }
            expression::SingleDataUnit::StructInit { .. } => {
                return Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::FeatureNotImplemented(
                        "Struct init".to_string(),
                    ),
                    tree,
                )])
            }
            expression::SingleDataUnit::ArrayAccess { .. } => {
                return Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::FeatureNotImplemented(
                        "Array access".to_string(),
                    ),
                    tree,
                )])
            }
            expression::SingleDataUnit::PropertyAccess { .. } => {
                return Err(vec![error::Error::from_cl_ln(
                    error::StaticAnalyzerErrorType::FeatureNotImplemented(
                        "Property access".to_string(),
                    ),
                    tree,
                )])
            }
        },
        expression::All::Cmp { value, .. } => match value {
            expression::Cmp::Equal { left, right, .. } => {
                return type_check!(Boolean, Equal, left, right, Integer, Float, String, Boolean);
            }
            expression::Cmp::NotEqual { left, right, .. } => {
                return type_check!(
                    Boolean, NotEqual, left, right, Integer, Float, String, Boolean
                );
            }
            expression::Cmp::LessThan { left, right, .. } => {
                return type_check!(Boolean, LessThan, left, right, Integer, Float);
            }
            expression::Cmp::LessThanOrEqual { left, right, .. } => {
                return type_check!(Boolean, LessThanOrEqual, left, right, Integer, Float);
            }
            expression::Cmp::GreaterThan { left, right, .. } => {
                return type_check!(Boolean, GreaterThan, left, right, Integer, Float);
            }
            expression::Cmp::GreaterThanOrEqual { left, right, .. } => {
                return type_check!(Boolean, GreaterThanOrEqual, left, right, Integer, Float);
            }
        },
        expression::All::BooleanLogic { value, .. } => match value {
            expression::BooleanLogic::And { left, right, .. } => {
                return type_check!(same ret, And, left, right, Boolean);
            }
            expression::BooleanLogic::Or { left, right, .. } => {
                return type_check!(same ret, Or, left, right, Boolean);
            }
            expression::BooleanLogic::Not { value, .. } => {
                let value = eval_type(&value, scope)?;

                if value == static_analyzer::Type::Boolean {
                    Ok(value)
                } else {
                    Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::OperationNotSupportedNot(value),
                        tree,
                    )])
                }
            }
        },
        expression::All::Arithmetic { value, .. } => match value {
            expression::Arithmetic::Add { left, right, .. } => {
                return type_check!(same ret, Add, left, right, Integer, Float, String);
            }
            expression::Arithmetic::Sub { left, right, .. } => {
                return type_check!(same ret, Sub, left, right, Integer, Float);
            }
            expression::Arithmetic::Mul { left, right, .. } => {
                return type_check!(same ret, Mul, left, right, Integer, Float);
            }
            expression::Arithmetic::Div { left, right, .. } => {
                return type_check!(same ret, Div, left, right, Integer, Float);
            }
            expression::Arithmetic::Mod { left, right, .. } => {
                return type_check!(same ret, Mod, left, right, Integer, Float);
            }
            expression::Arithmetic::Pow { left, right, .. } => {
                return type_check!(same ret, Pow, left, right, Integer, Float);
            }
            expression::Arithmetic::Neg { value, .. } => {
                let value = eval_type(&value, scope)?;

                if value == static_analyzer::Type::Integer || value == static_analyzer::Type::Float
                {
                    Ok(value)
                } else {
                    Err(vec![error::Error::from_cl_ln(
                        error::StaticAnalyzerErrorType::OperationNotSupportedNeg(value),
                        tree,
                    )])
                }
            }
        },
    }
}
