use crate::Associativity;

use super::{Engine, Expr, List, Operator, SymErr, Symbol, Tree};

fn split_keep<'a>(str: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in str.match_indices(Operator::is_operator) {
        if last != index {
            result.push(&str[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < str.len() {
        result.push(&str[last..]);
    }

    result
}

pub fn parse_infix(infix_string: &str) -> Result<Vec<Symbol>, SymErr> {
    let py_fixed_str = infix_string.replace("**", "^");
    let infix_split = split_keep(py_fixed_str.as_str());
    Ok(infix_split
        .iter()
        .enumerate()
        .filter_map(|(i, &to_parse)| {
            if let Ok(number) = to_parse.parse() {
                Some(Ok(Symbol::Number(number)))
            } else if let Ok(oper) = Operator::from(to_parse.chars().next().unwrap()) {
                let is_sign = i == 0
                    || infix_split[i - 1] == "("
                    || infix_split[i - 1] == "-"
                    || infix_split[i - 1] == "+";
                if is_sign && oper == Operator::Add {
                    None
                } else if is_sign && oper == Operator::Sub {
                    Some(Ok(Symbol::Negate(())))
                } else if is_sign && !oper.is_parenthesis() {
                    Some(Err(SymErr::InvalidSign))
                } else {
                    Some(Ok(Symbol::Operator(oper)))
                }
            } else {
                Some(Ok(Symbol::String(String::from(to_parse))))
            }
        })
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn to_postfix(engine: &Engine, infix: &Vec<Symbol>) -> Result<Vec<Symbol>, SymErr> {
    let mut postfix = Vec::new();
    let mut operator_stack = Vec::new();
    let mut last_was_number = false;

    if engine.debugging {
        println!("To postfix: {:?}", infix);
    }

    for (i, symbol) in infix.iter().enumerate() {
        match symbol {
            Symbol::Number(_) => {
                postfix.push(symbol.clone());
                last_was_number = true;
            }
            Symbol::Negate(_) => operator_stack.push(symbol.clone()),
            Symbol::Operator(oper) => {
                if *oper == Operator::LPa {
                    if last_was_number {
                        operator_stack.push(Symbol::Operator(Operator::Mul));
                    }
                    operator_stack.push(symbol.clone());
                } else if *oper == Operator::RPa {
                    loop {
                        if engine.debugging {
                            println!("");
                            println!("-------- {:?} at {} --------", symbol, i);
                            println!("operator_stack = {:?}", operator_stack);
                            println!("postfix = {:?}", postfix);
                            println!("infix = {:?}", infix);
                        }

                        let top_symbol = match operator_stack.pop() {
                            Some(Symbol::Operator(top_operator)) => {
                                if top_operator == Operator::LPa {
                                    break;
                                }
                                Symbol::Operator(top_operator)
                            }
                            Some(Symbol::Negate(_)) => Symbol::Negate(()),
                            Some(sym) => sym.clone(),
                            None => return Err(SymErr::ParenthesesMismatch),
                        };

                        postfix.push(top_symbol.clone());
                    }

                    match operator_stack.pop() {
                        Some(Symbol::String(string)) => {
                            if engine.functions.contains_key(string.as_str()) {
                                postfix.push(Symbol::String(string.clone()));
                            }
                        }
                        Some(symbol) => operator_stack.push(symbol),
                        None => (),
                    }
                } else {
                    let mut last_oper = *oper;
                    loop {
                        let top_symbol = operator_stack.pop();
                        let top_operator = match top_symbol {
                            Some(Symbol::Operator(oper)) => oper,
                            Some(Symbol::Negate(_)) => Operator::Custom(4, Associativity::Left),
                            _ => break,
                        };

                        if top_operator.is_parenthesis()
                            || top_operator.precedence().unwrap() < last_oper.precedence().unwrap()
                        {
                            operator_stack.push(top_symbol.unwrap());
                            break;
                        }

                        last_oper = top_operator;
                        match top_operator {
                            Operator::Custom(_, _) => postfix.push(Symbol::Negate(())),
                            _ => postfix.push(Symbol::Operator(top_operator)),
                        }
                    }
                    operator_stack.push(symbol.clone())
                }
                last_was_number = false;
            }
            Symbol::String(_) => {
                postfix.push(symbol.clone());
                last_was_number = false;
            }
        }
    }

    loop {
        match operator_stack.pop() {
            Some(symbol) => postfix.push(symbol.clone()),
            _ => break,
        }
    }

    Ok(postfix)
}

pub fn postfix_to_tree(postfix: &Vec<Symbol>) -> Result<Expr, SymErr> {
    let mut mixed_stack = Vec::new();

    for symbol in postfix.iter() {
        match symbol {
            Symbol::Operator(op) => {
                let a = mixed_stack.pop().ok_or(SymErr::StackEmpty)?;
                let b = mixed_stack.pop().ok_or(SymErr::StackEmpty)?;

                mixed_stack.push(Expr::Operator(Tree {
                    value: op.clone(),
                    next: Some((Box::new(b), Box::new(a))),
                }));
            }
            Symbol::Negate(_) => {
                let a = mixed_stack.pop().ok_or(SymErr::StackEmpty)?;

                mixed_stack.push(Expr::Negate(List {
                    value: (),
                    next: Some(Box::new(a)),
                }));
            }
            Symbol::Number(n) => {
                mixed_stack.push(Expr::Number(n.clone()));
            }
            Symbol::String(s) => {
                mixed_stack.push(Expr::String(s.clone()));
            }
        }
    }

    assert!(mixed_stack.len() == 1, "Postfix had leftover symbols");

    Ok(mixed_stack.pop().unwrap())
}
