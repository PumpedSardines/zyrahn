fn main() {
    let code: &'static str = "test::1";
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
