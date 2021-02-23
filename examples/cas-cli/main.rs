use cas::{Engine, Expr};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::stdout;

const DEBUG: bool = true;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let engine = if DEBUG {
        Engine::new().with_functions().with_debugging()
    } else {
        Engine::new().with_functions()
    };
    let stdout = stdout();

    for arg in args.iter() {
        if arg.len() == 0 {
            continue;
        }

        let expr = Expr::parse(&engine, arg).unwrap();
        let simple = expr.simplify(&engine);
        let eval = simple.eval();

        execute!(
            &stdout,
            Print("Parsed: "),
            SetForegroundColor(Color::Green),
            Print(format!("{}", expr)),
            ResetColor,
            Print(" latex: "),
            SetForegroundColor(Color::Cyan),
            Print(format!("{}\n", expr.print_latex())),
            ResetColor,
            Print("Simplified: "),
            SetForegroundColor(Color::Green),
            Print(format!("{}", simple)),
            ResetColor,
            Print(" latex: "),
            SetForegroundColor(Color::Cyan),
            Print(format!("{}\n", simple.print_latex())),
            ResetColor,
            Print("Evaluated: "),
            SetForegroundColor(Color::Green),
            Print(format!("{}", eval)),
            ResetColor,
            Print(" latex: "),
            SetForegroundColor(Color::Cyan),
            Print(format!("{}\n", eval.print_latex())),
            ResetColor
        )
        .unwrap();
    }

    Ok(())
}
