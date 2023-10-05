fn compile(code: &str) -> Result<zyrahn::ast::node::expression::All, Box<dyn std::error::Error>> {
    let tokens = zyrahn::lexer::tokenize(code)?;
    let ast = zyrahn::ast::gen(&tokens)?;

    if let Err(e) = zyrahn::static_analyzer::check(&ast) {
        return Err(Box::new(e));
    }

    Ok(ast)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <code>", args[0]);
        std::process::exit(1);
    }

    let ast = compile(&args[1]).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    println!("{:#?}", ast);
}
