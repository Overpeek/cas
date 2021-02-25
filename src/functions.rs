use crate::{Engine, Expr, FnMap, Number, SymErr};

pub fn ln(engine: &Engine, arguments: &Vec<Box<Expr>>) -> Result<Expr, SymErr> {
    if arguments.len() != 1 {
        Err(SymErr::InvalidFunctionArgCount)
    } else {
        let arg_0 = arguments.get(0).unwrap().as_ref().eval(engine)?;

        match arg_0 {
            Expr::Number(Number::Rational(0, 1)) => Err(SymErr::Undefined),
            Expr::Number(Number::Rational(1, 1)) => Ok(Expr::from(0)),
            /* TODO: ln(e) = 1 Expr::Number(Number::Rational(1, 1)) => {
                Ok(vec![Box::new(Expr::Number(Number::Rational(0, 1)))])
            } */
            Expr::Number(n) => {
                let f: f64 = n.clone().into();
                Ok(Expr::from(f.ln()))
            }
            _ => Expr::func("ln", vec![arg_0.clone()]),
        }
    }
}

pub fn all(map: &mut FnMap) {
    map.insert("ln", (1, ln));
}
