fn compile(code: &str) -> Result<zyrahn::ast::node::expression::All, Box<dyn std::error::Error>> {
    let tokens = zyrahn::lexer::tokenize(code)?;
    let ast = zyrahn::ast::gen(&tokens)?;

    Ok(ast)
}

fn main() {
    let ast = compile("test()").unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });
    println!("{:#?}", ast);
}
