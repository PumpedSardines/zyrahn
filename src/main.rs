fn compile(
    code: &str,
) -> Result<Vec<zyrahn::ast::node::block::All>, Vec<Box<dyn std::error::Error>>> {
    let tokens = zyrahn::lexer::tokenize(code);
    if let Err(e) = &tokens {
        return Err(vec![Box::new(e.clone())]);
    }
    let tokens = tokens.unwrap();

    let ast = zyrahn::ast::gen(&tokens);
    if let Err(e) = &ast {
        return Err(vec![Box::new(e.clone())]);
    }
    let ast = ast.unwrap();

    if let Err(errs) = zyrahn::static_analyzer::check(&ast) {
        let mut out_errs: Vec<Box<dyn std::error::Error>> = vec![];
        for e in errs {
            out_errs.push(Box::new(e.clone()));
        }
        return Err(out_errs);
    }

    Ok(ast)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <code>", args[0]);
        std::process::exit(1);
    }

    let ast = compile(&args[1]).unwrap_or_else(|errs| {
        for e in errs {
            println!("{}", e);
        }
        std::process::exit(1);
    });

    println!("{:#?}", ast);
}
