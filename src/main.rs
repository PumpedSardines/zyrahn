// const STD_LIBRARY: &str = include_str!("./std.zy");

fn compile(code: &str) -> Result<String, Vec<Box<dyn std::error::Error>>> {
    // let code = format!("{}\n\n{}", code, STD_LIBRARY);

    let tokens = zyrahn::lexer::tokenize(&code);
    if let Err(e) = &tokens {
        return Err(vec![Box::new(e.clone())]);
    }
    let tokens = tokens.unwrap();

    let ast = zyrahn::parser::gen(&tokens);
    if let Err(e) = &ast {
        return Err(vec![Box::new(e.clone())]);
    }
    let ast = ast.unwrap();

    let typed_ast = zyrahn::static_analyzer::evaluate(&ast);

    if let Err(errs) = typed_ast {
        let mut out_errs: Vec<Box<dyn std::error::Error>> = vec![];
        for e in errs {
            out_errs.push(Box::new(e.clone()));
        }
        return Err(out_errs);
    }

    Ok(zyrahn::compiler::javascript::compile(&typed_ast.unwrap()))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <code>", args[0]);
        std::process::exit(1);
    }

    let js = compile(&args[1]).unwrap_or_else(|errs| {
        for e in errs {
            println!("{}", e);
        }
        std::process::exit(1);
    });

    println!("{:#?}", js);
}
