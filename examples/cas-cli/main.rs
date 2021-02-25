use cas::{Engine, Expr, SymErr};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::stdout;

const DEBUG: bool = false;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let engine = if DEBUG {
        Engine::new().with_functions().with_debugging()
    } else {
        Engine::new().with_functions()
    };
    let stdout = stdout();

    let handle = |arg: &str| -> Result<(), SymErr> {
        let expr = Expr::parse(&engine, arg.replace(|c: char| c == 27 as char, "").as_str())?;
        let simple = expr.simplify(&engine);
        let eval = simple.eval(&engine)?;

        execute!(
            &stdout,
            Print("Parsed: "),
            SetForegroundColor(Color::Green),
            Print(format!("{}", expr)),
            ResetColor,
            Print(" latex: "),
            SetForegroundColor(Color::Cyan),
            Print(format!("{}", expr.print_latex())),
            ResetColor,
            Print(" debug: "),
            SetForegroundColor(Color::Yellow),
            Print(format!("{}\n", expr.print_debug())),
            ResetColor,
            Print("Simplified: "),
            SetForegroundColor(Color::Green),
            Print(format!("{}", simple)),
            ResetColor,
            Print(" latex: "),
            SetForegroundColor(Color::Cyan),
            Print(format!("{}", simple.print_latex())),
            ResetColor,
            Print(" debug: "),
            SetForegroundColor(Color::Yellow),
            Print(format!("{}\n", simple.print_debug())),
            ResetColor,
            Print("Evaluated: "),
            SetForegroundColor(Color::Green),
            Print(format!("{}", eval)),
            ResetColor,
            Print(" latex: "),
            SetForegroundColor(Color::Cyan),
            Print(format!("{}", eval.print_latex())),
            ResetColor,
            Print(" debug: "),
            SetForegroundColor(Color::Yellow),
            Print(format!("{}\n", eval.print_debug())),
            ResetColor
        )
        .unwrap();

        Ok(())
    };

    for arg in args.iter() {
        handle(arg).unwrap_or_else(|err| println!("{:?}", err));
    }

    if args.len() == 0 {
        let mut buf = String::new();
        println!("Input:");
        loop {
            std::io::stdin().read_line(&mut buf).unwrap();
            handle(buf.trim_end()).unwrap_or_else(|err| println!("{:?}", err));
            buf.clear();
        }
    }

    Ok(())
}
