mod expression;
pub mod node;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot parse expression")]
    EvaluateExpression,
}

pub fn gen(tokens: &Vec<lexer::Token>) {
    let ast = expression::gen(&tokens);
    if let Err(e) = ast {
        println!("{}", e);
        return;
    }
    let ast = ast.unwrap();
    let data = serde_json::to_string_pretty(&ast).unwrap();

    println!("{}", data);
}
