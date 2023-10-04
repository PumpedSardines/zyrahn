fn main() {
    let code: &'static str = "example**(8-3) == 3 && !true || false";
    let tokens = lexer::tokenize(code).unwrap();

    println!(
        "{:?}",
        // tokens,
        tokens
            .clone()
            .into_iter()
            .map(|t| t.token_type)
            .collect::<Vec<lexer::TokenType>>()
    );
    ast::gen(&tokens);
}
