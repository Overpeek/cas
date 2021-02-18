mod core;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("CLI: {:?}", args);

    let debugging = if let Ok(debug_flag) = std::env::var("CAS_DEBUG") {
        debug_flag == "true"
            || debug_flag == "True"
            || debug_flag == "1"
            || debug_flag == "on"
            || debug_flag == "ON"
    } else {
        false
    };

    let engine = if debugging {
        core::Engine::new().with_functions().with_debugging()
    } else {
        core::Engine::new().with_functions()
    };
    let parse_eval_print = |input: &str| {
        if input.len() == 0 {
            return;
        }

        println!("{}", "-".repeat(7 + input.len()));
        println!("Input: {}", input);

        let expr = engine.parse_infix(input).unwrap();
        println!("Parsed: {:?}", expr);

        let eval = expr.eval().unwrap();
        let evalf = expr.evalf().unwrap();

        if eval.stack.len() == 1 {
            println!("Evaluated: {:?}", eval.stack.first().unwrap());
        } else {
            println!("Evaluated: {:?}", eval);
        }

        if evalf.stack.len() == 1 {
            println!("Force evaluated: {:?}", evalf.stack.first().unwrap());
        } else {
            println!("Force evaluated: {:?}", evalf);
        }
    };

    for arg in args.iter() {
        parse_eval_print(arg);
    }

    Ok(())
}
