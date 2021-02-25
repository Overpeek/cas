use crate::Engine;

use super::{Expr, Operator, SymErr};

pub fn eval_tree(engine: &Engine, tree: &Expr) -> Result<Expr, SymErr> {
    match &tree {
        Expr::Function(f) => {
            if let Some(function) = engine.functions.get(f.value.as_str()) {
                function.1(engine, f.next.as_ref().unwrap())
            } else {
                Err(SymErr::UnknownFunction)
            }
        }
        Expr::Operator(o) => match o.value {
            Operator::Pos | Operator::Neg => {
                let value = eval_tree(engine, o.next.as_ref().unwrap()[0].as_ref())?;

                let string = format!("Evaluating, {}({})", o.value.to(), value);
                let result = value.operate(o.value, None).unwrap();
                if engine.debugging {
                    println!("{} = {}", string, result);
                }
                Ok(result)
            }
            _ => {
                let left = eval_tree(engine, o.next.as_ref().unwrap()[0].as_ref())?;
                let right = eval_tree(engine, o.next.as_ref().unwrap()[1].as_ref())?;

                let string = format!("Evaluating, {}{}{}", left, o.value.to(), right);
                let result = left.operate(o.value, Some(right)).unwrap();

                if engine.debugging {
                    println!("{} = {}", string, result);
                }

                Ok(result)
            }
        },
        _ => Ok(tree.clone()),
    }
}
