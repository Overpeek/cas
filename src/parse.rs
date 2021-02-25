use crate::Number;

use super::{Engine, Expr, Operator, SymErr, Symbol, Tree};

fn split_keep<'a>(str: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in str.match_indices(|c| Operator::is_operator(c) || c == ',') {
        if last != index {
            result.push(&str[last..index]);
        }
        if matched != "," {
            result.push(matched);
        }
        last = index + matched.len();
    }
    if last < str.len() {
        result.push(&str[last..]);
    }

    result
}

pub fn parse_infix(engine: &Engine, infix_string: &str) -> Result<Vec<Symbol>, SymErr> {
    let py_fixed_str = infix_string.replace("**", "^").replace(' ', "");
    let infix_split = split_keep(py_fixed_str.as_str());

    if engine.debugging {
        println!("To infix: {}", infix_string);
    }

    Ok(infix_split
        .iter()
        .enumerate()
        .map(|(i, &to_parse)| {
            if let Ok(number) = Number::parse(to_parse) {
                Ok(Symbol::Number(number))
            } else if let Ok(oper) = Operator::from(to_parse.chars().next().unwrap()) {
                let is_sign = i == 0
                    || infix_split[i - 1] == "("
                    || infix_split[i - 1] == "-"
                    || infix_split[i - 1] == "+";
                if is_sign && oper == Operator::Add {
                    Ok(Symbol::Operator(Operator::Pos))
                } else if is_sign && oper == Operator::Sub {
                    Ok(Symbol::Operator(Operator::Neg))
                } else if is_sign && !oper.is_parenthesis() {
                    Err(SymErr::InvalidSign)
                } else {
                    Ok(Symbol::Operator(oper))
                }
            } else {
                if engine.functions.contains_key(to_parse) {
                    Ok(Symbol::Function(String::from(to_parse)))
                } else {
                    Ok(Symbol::Variable(String::from(to_parse)))
                }
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

    for symbol in infix.iter() {
        last_was_number = match symbol {
            Symbol::Number(_) => {
                postfix.push(symbol.clone());
                true
            }
            Symbol::Variable(_) => {
                postfix.push(symbol.clone());
                false
            }
            Symbol::Function(_) => {
                operator_stack.push(symbol.clone());
                false
            }
            Symbol::Operator(Operator::LPa) => {
                if last_was_number {
                    operator_stack.push(Symbol::Operator(Operator::Mul));
                }
                operator_stack.push(symbol.clone());
                false
            }
            Symbol::Operator(Operator::RPa) => {
                loop {
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
                    Some(Symbol::Function(string)) => {
                        postfix.push(Symbol::Function(string.clone()));
                    }
                    Some(symbol) => {
                        operator_stack.push(symbol);
                    }
                    None => (),
                }
                false
            }
            Symbol::Operator(oper) => {
                let mut operator = *oper;

                loop {
                    let top_symbol = operator_stack.pop();
                    let top_operator = match top_symbol {
                        Some(Symbol::Operator(oper)) => oper,
                        _ => break,
                    };

                    if top_operator.is_parenthesis()
                        || top_operator.precedence().unwrap() <= operator.precedence().unwrap()
                    {
                        operator_stack.push(top_symbol.unwrap());
                        break;
                    }

                    operator = top_operator;
                    postfix.push(Symbol::Operator(top_operator));
                }
                operator_stack.push(symbol.clone());
                false
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

pub fn postfix_to_tree(engine: &Engine, postfix: &Vec<Symbol>) -> Result<Expr, SymErr> {
    let mut mixed_stack: Vec<Expr> = Vec::new();

    if engine.debugging {
        println!("To expr tree: {:?}", postfix);
    }

    for symbol in postfix.iter() {
        match symbol {
            Symbol::Operator(Operator::Neg) => {
                let a = mixed_stack.pop().ok_or(SymErr::StackEmpty)?;
                mixed_stack.push(-a);
            }
            Symbol::Operator(Operator::Pos) => {
                let a = mixed_stack.pop().ok_or(SymErr::StackEmpty)?;
                mixed_stack.push(a);
            }
            Symbol::Operator(op) => {
                let a = mixed_stack.pop().ok_or(SymErr::StackEmpty)?;
                let b = mixed_stack.pop().ok_or(SymErr::StackEmpty)?;

                mixed_stack.push(Expr::Operator(Tree {
                    value: op.clone(),
                    next: Some(vec![Box::new(b), Box::new(a)]),
                }));
            }
            Symbol::Number(n) => {
                mixed_stack.push(Expr::Number(n.clone()));
            }
            Symbol::Variable(s) => {
                mixed_stack.push(Expr::Variable(s.clone()));
            }
            Symbol::Function(s) => {
                let &(argc, _) = engine
                    .functions
                    .get(s.as_str())
                    .ok_or(SymErr::UnknownFunction)?;

                let mut arguments = Vec::new();
                for _ in 0..argc {
                    arguments.push(Box::new(
                        mixed_stack.pop().ok_or(SymErr::InvalidFunctionArgCount)?,
                    ));
                }

                mixed_stack.push(Expr::Function(Tree {
                    value: s.clone(),
                    next: Some(arguments),
                }));
            }
        }
    }

    assert!(mixed_stack.len() == 1, "Postfix had leftover symbols");

    Ok(mixed_stack.pop().unwrap())
}

enum Ordering {
    Left,
    Right,
    Both,
    Neither,
}

impl Ordering {
    pub fn new(left: u8, oper: u8, right: u8) -> Self {
        // println!("orders {} {} {}", left, oper, right);
        if left >= oper && oper <= right {
            Self::Neither
        } else if left >= oper {
            Self::Right
        } else if left < oper && oper <= right {
            Self::Left
        } else {
            Self::Both
        }
    }
}

fn tree_to_infix_recurse(expr: &Expr) -> (String, u8) {
    match &expr {
        Expr::Function(f) => (
            format!(
                "{}({})",
                f.value,
                (|| {
                    let mut l = String::new();
                    f.next.as_ref().unwrap().iter().for_each(|e| {
                        l = format!("{}, {}", l, tree_to_infix(e));
                    });

                    if l.is_empty() {
                        l
                    } else {
                        String::from(&l[2..])
                    }
                })()
            ),
            u8::MAX,
        ),
        Expr::Operator(o) => match o.value {
            Operator::Pos => (
                format!(
                    "-({})",
                    tree_to_infix_recurse(&o.next.as_ref().unwrap()[0]).0
                ),
                4,
            ),
            Operator::Neg => (
                format!("{}", tree_to_infix_recurse(&o.next.as_ref().unwrap()[0]).0),
                4,
            ),
            _ => {
                let a = tree_to_infix_recurse(&o.next.as_ref().unwrap()[0]);
                let b = tree_to_infix_recurse(&o.next.as_ref().unwrap()[1]);
                let c = o.value.precedence().unwrap();

                match Ordering::new(a.1, c, b.1) {
                    Ordering::Neither => (format!("{}{}{}", a.0, o.value.to(), b.0), c),
                    Ordering::Right => (format!("{}{}({})", a.0, o.value.to(), b.0), c),
                    Ordering::Left => (format!("({}){}{}", a.0, o.value.to(), b.0), c),
                    Ordering::Both => (format!("({}){}({})", a.0, o.value.to(), b.0), c),
                }
            }
        },
        Expr::Variable(v) => (format!("{}", v), u8::MAX),
        Expr::Number(n) => (format!("{}", n), u8::MAX),
        Expr::Identifier(i) => (format!("i:{}", i), u8::MAX),
    }
}

pub fn tree_to_infix(expr: &Expr) -> String {
    tree_to_infix_recurse(expr).0
}

fn tree_to_latex_recurse(expr: &Expr) -> (String, u8) {
    match &expr {
        Expr::Function(f) => (
            format!(
                "\\{}\\left({}\\right)",
                f.value,
                (|| {
                    let mut l = String::new();
                    f.next.as_ref().unwrap().iter().for_each(|e| {
                        l = format!("{}, {}", l, tree_to_latex_recurse(e).0);
                    });

                    if l.is_empty() {
                        l
                    } else {
                        String::from(&l[2..])
                    }
                })()
            ),
            u8::MAX,
        ),
        Expr::Operator(o) => match o.value {
            Operator::Pos => (
                format!(
                    "-({})",
                    tree_to_latex_recurse(&o.next.as_ref().unwrap()[0]).0
                ),
                4,
            ),
            Operator::Neg => (
                format!("{}", tree_to_latex_recurse(&o.next.as_ref().unwrap()[0]).0),
                4,
            ),
            _ => {
                let a = tree_to_latex_recurse(&o.next.as_ref().unwrap()[0]);
                let b = tree_to_latex_recurse(&o.next.as_ref().unwrap()[1]);
                let c = o.value.precedence().unwrap();

                match o.value {
                    Operator::Div => (format!("\\frac{{{}}}{{{}}}", a.0, b.0), c),
                    Operator::Mul => match Ordering::new(a.1, c, b.1) {
                        Ordering::Neither => (format!("{}\\cdot{}", a.0, b.0), c),
                        Ordering::Right => (format!("{}\\cdot\\left({}\\left)", a.0, b.0), c),
                        Ordering::Left => (format!("\\left({}\\left)\\cdot{}", a.0, b.0), c),
                        Ordering::Both => (
                            format!("\\left({}\\left)\\cdot\\left({}\\left)", a.0, b.0),
                            c,
                        ),
                    },
                    Operator::Pow => match Ordering::new(a.1, c, b.1) {
                        Ordering::Neither => (format!("{}^{}", a.0, b.0), c),
                        Ordering::Right => (format!("{}^{{{}}}", a.0, b.0), c),
                        Ordering::Left => (format!("\\left({}\\left)^{}", a.0, b.0), c),
                        Ordering::Both => (format!("\\left({}\\left)^{{{}}}", a.0, b.0), c),
                    },
                    _ => match Ordering::new(a.1, c, b.1) {
                        Ordering::Neither => (format!("{}{}{}", a.0, o.value.to(), b.0), c),
                        Ordering::Right => {
                            (format!("{}{}\\left({}\\left)", a.0, o.value.to(), b.0), c)
                        }
                        Ordering::Left => {
                            (format!("\\left({}\\left){}{}", a.0, o.value.to(), b.0), c)
                        }
                        Ordering::Both => (
                            format!("\\left({}\\left){}\\left({}\\left)", a.0, o.value.to(), b.0),
                            c,
                        ),
                    },
                }
            }
        },
        Expr::Variable(v) => (format!("{}", v), u8::MAX),
        Expr::Number(n) => (format!("{}", n), u8::MAX),
        Expr::Identifier(i) => (format!("i:{}", i), u8::MAX),
    }
}

pub fn tree_to_latex(expr: &Expr) -> String {
    tree_to_latex_recurse(expr).0
}
