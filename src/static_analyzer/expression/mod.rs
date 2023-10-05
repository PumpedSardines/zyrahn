use crate::*;

pub fn eval_type(
    tree: &ast::node::expression::All,
    scope: &mut static_analyzer::Scope,
) -> Result<static_analyzer::r#type::Type, error::Error<error::StaticAnalyzerErrorType>> {
    todo!()
}
