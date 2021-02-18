use super::{Engine, Operator, SymErr, Symbol};

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

pub fn parse_infix(infix_string: &str) -> Vec<Symbol> {
    let infix = split_keep(infix_string);
    infix
        .iter()
        .map(|&to_parse| {
            if let Ok(number) = to_parse.parse() {
                Symbol::Number(number)
            } else if let Ok(oper) = Operator::from(to_parse.chars().next().unwrap()) {
                Symbol::Operator(oper)
            } else {
                Symbol::String(String::from(to_parse))
            }
        })
        .collect::<Vec<_>>()
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
                            _ => break,
                        };

                        if top_operator.is_parenthesis()
                            || top_operator.precedence().unwrap() < last_oper.precedence().unwrap()
                        {
                            operator_stack.push(top_symbol.unwrap());
                            break;
                        }

                        last_oper = top_operator;
                        postfix.push(Symbol::Operator(top_operator));
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

enum InfixPostfixMix {
    Postfix(Symbol),
    Infix(Vec<Symbol>),
}

impl InfixPostfixMix {
    fn operate(self, rhs: InfixPostfixMix, oper: Operator) -> InfixPostfixMix {
        let mut vec_left = match self {
            InfixPostfixMix::Postfix(pfx) => vec![pfx],
            InfixPostfixMix::Infix(ifx) => ifx,
        };
        let vec_rhs = match rhs {
            InfixPostfixMix::Postfix(pfx) => vec![pfx],
            InfixPostfixMix::Infix(ifx) => ifx,
        };

        vec_left.insert(0, Symbol::Operator(Operator::LPa));
        vec_left.push(Symbol::Operator(oper));
        for symbol in vec_rhs {
            vec_left.push(symbol);
        }
        vec_left.push(Symbol::Operator(Operator::RPa));
        InfixPostfixMix::Infix(vec_left)
    }
}

pub fn to_infix(postfix: &Vec<Symbol>) -> Result<Vec<Symbol>, SymErr> {
    let mut stack: Vec<InfixPostfixMix> = Vec::new();
    for symbol in postfix.iter() {
        if let Symbol::Operator(oper) = symbol {
            let a = stack.pop().ok_or(SymErr::InvalidOP)?;
            let b = stack.pop().ok_or(SymErr::InvalidOP)?;

            stack.push(a.operate(b, *oper));
        } else {
            stack.push(InfixPostfixMix::Postfix(symbol.clone()));
        }
    }

    match stack.pop() {
        Some(InfixPostfixMix::Infix(ifx)) => Ok(ifx),
        Some(InfixPostfixMix::Postfix(pfx)) => Ok(vec![pfx]),
        None => Err(SymErr::StackNotLengthOne),
    }
}
