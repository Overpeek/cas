use std::ops::Rem;

use super::{Engine, Operator, SymErr, Symbol};

pub fn eval_postfix(
    engine: &Engine,
    postfix: Vec<Symbol>,
    force: bool,
) -> Result<Vec<Symbol>, SymErr> {
    let mut stack = Vec::new();

    if engine.debugging {
        println!("Evaluating: {:?}", postfix);
    }

    for symbol in postfix.iter() {
        match symbol {
            Symbol::Operator(oper) => {
                let a = stack.pop().ok_or(SymErr::StackEmpty)?;
                let b = stack.pop().ok_or(SymErr::StackEmpty)?;

                if engine.debugging {
                    println!("Evaluating: {:?} {:?} {:?}", a, b, symbol);
                }

                match (a, b) {
                    (Symbol::Number(number_a), Symbol::Number(number_b)) => match oper {
                        Operator::Add => stack.push(Symbol::Number(number_b + number_a)),
                        Operator::Sub => stack.push(Symbol::Number(number_b - number_a)),
                        Operator::Div => {
                            if force || (number_b as i64).rem(number_a as i64) == 0 {
                                stack.push(Symbol::Number(number_b / number_a))
                            } else {
                                stack.push(Symbol::Number(number_b));
                                stack.push(Symbol::Number(number_a));
                                stack.push(symbol.clone())
                            }
                        }
                        Operator::Mul => stack.push(Symbol::Number(number_b * number_a)),
                        Operator::Pow => stack.push(Symbol::Number(number_b.powf(number_a))),
                        _ => return Err(SymErr::InvalidOP),
                    },
                    (a, b) => {
                        stack.push(b);
                        stack.push(a);
                        stack.push(symbol.clone());
                    }
                }
            }
            _ => stack.push(symbol.clone()),
        }
    }

    Ok(stack)
}
