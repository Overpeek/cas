use cas::Engine;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let engine = Engine::new().with_functions()/* .with_debugging() */;

    let parse_eval_print = |input: &str| {
        if input.len() == 0 {
            return;
        }

        let expr = engine.parse(input).unwrap();
        
		println!("Parsed: {}", expr);
        println!("Evaluated: {}", expr.eval());
    };

    for arg in args.iter() {
        parse_eval_print(arg);
    }

    Ok(())
}
