use std::process::exit;

use cas::Engine;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let invalid_args = || {
        println!("Usage: cas-cli -/-v/-vv EXPR1 EXPR2 EXPR3 ...");
        exit(-1)
    };

    let verbosity = match args.first().unwrap_or_else(invalid_args).as_str() {
        "-v" => 1,
        "-vv" => 2,
        _ => 0, // -, -e, or anything
    };

    let engine = if verbosity == 2 {
        println!("CLI: {:?}", args);
        Engine::new().with_functions().with_debugging()
    } else {
        Engine::new().with_functions()
    };
    let parse_eval_print = |input: &str| {
        if input.len() == 0 {
            return;
        }

        let expr = engine.parse_infix(input).unwrap();
        let evalf = expr.evalf().unwrap();

        if verbosity == 0 {
            println!("{}", evalf.print_infix().unwrap());
        } else {
            let eval = expr.eval().unwrap();

            println!("{}", "-".repeat(7 + input.len()));
            println!("Input: {}", input);

            println!(
                "Parsed: {} (postfix: {})",
                expr.print_infix().unwrap(),
                expr.print_postfix().unwrap()
            );

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
        }
    };

    for arg in args.iter().skip(1) {
        parse_eval_print(arg);
    }

    Ok(())
}
