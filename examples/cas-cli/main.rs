use cas::{Engine, Expr};

const DEBUG: bool = true;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let engine = if DEBUG {
        Engine::new().with_functions().with_debugging()
    } else {
        Engine::new().with_functions()
    };

    let parse_eval_print = |input: &str| {
        if input.len() == 0 {
            return;
        }

        let expr = Expr::parse(&engine, input).unwrap();

        println!("Parsed: {}", expr);
        println!("Evaluated: {}", expr.eval());
        println!("Simplified: {}", expr.simplify(&engine));
    };

    for arg in args.iter() {
        parse_eval_print(arg);
    }

    Ok(())
}
