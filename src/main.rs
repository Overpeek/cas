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
        println!(
            "Parsed: {} (postfix: {})",
            expr.print_infix().unwrap(),
            expr.print_postfix().unwrap()
        );

        let eval = expr.eval().unwrap();
        let evalf = expr.evalf().unwrap();

        println!(
            "Evaluated: {} (postfix: {})",
            eval.print_infix().unwrap(),
            eval.print_postfix().unwrap()
        );

        println!(
            "Force evaluated: {}, (postfix: {})",
            evalf.print_infix().unwrap(),
            evalf.print_postfix().unwrap()
        );
    };

    for arg in args.iter() {
        parse_eval_print(arg);
    }

    Ok(())
}
