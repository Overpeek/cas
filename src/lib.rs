use std::{borrow::Borrow, collections::HashMap};

mod constants;
mod eval;
mod functions;
mod parse;
mod simplifier;

type Stack = Vec<Symbol>;

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Number(f64),
    String(String),
    Operator(Operator),
}

#[derive(Debug)]
pub enum SymErr {
    StackEmpty,
    NotANumber,
    InvalidOP,
    InvalidSign,
    ParenthesesMismatch,
    StackNotLengthOne,
    Inconvertible,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Pos, // +
    Neg, // -
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Pow, // ^ (TODO: or **)
    LPa, // (
    RPa, // )
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Associativity {
    Left,
    Right,
}

impl Operator {
    pub fn is_parenthesis(&self) -> bool {
        *self == Operator::LPa || *self == Operator::RPa
    }

    pub fn is_sign(&self) -> bool {
        *self == Operator::Pos || *self == Operator::Neg
    }

    pub fn is_signable(&self) -> bool {
        self.is_sign() || *self == Operator::Add || *self == Operator::Sub
    }

    pub fn precedence(&self) -> Result<u8, SymErr> {
        match self {
            Operator::Pos => Ok(4),
            Operator::Neg => Ok(4),
            Operator::Add => Ok(2),
            Operator::Sub => Ok(2),
            Operator::Div => Ok(3),
            Operator::Mul => Ok(3),
            Operator::Pow => Ok(5),
            _ => Err(SymErr::InvalidOP),
        }
    }

    pub fn associativity(&self) -> Result<Associativity, SymErr> {
        match self {
            Operator::LPa | Operator::RPa => Err(SymErr::InvalidOP),
            Operator::Pow => Ok(Associativity::Right),
            _ => Ok(Associativity::Left),
        }
    }

    pub fn to(&self) -> char {
        match self {
            Operator::Pos => '+',
            Operator::Neg => '-',
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Div => '/',
            Operator::Mul => '*',
            Operator::Pow => '^',
            Operator::LPa => '(',
            Operator::RPa => ')',
        }
    }

    pub fn to_sign(&self) -> Result<Self, SymErr> {
        match self {
            Operator::Pos => Ok(Operator::Pos),
            Operator::Neg => Ok(Operator::Neg),
            Operator::Add => Ok(Operator::Pos),
            Operator::Sub => Ok(Operator::Neg),
            _ => Err(SymErr::Inconvertible),
        }
    }

    pub fn from(c: char) -> Result<Self, SymErr> {
        match c {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Sub),
            '*' => Ok(Operator::Mul),
            '/' => Ok(Operator::Div),
            '^' => Ok(Operator::Pow),
            '(' => Ok(Operator::LPa),
            ')' => Ok(Operator::RPa),
            _ => Err(SymErr::InvalidOP),
        }
    }

    pub fn is_operator(c: char) -> bool {
        c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '(' || c == ')'
    }
}

pub struct Engine<'a> {
    functions: HashMap<&'a str, fn(&mut Stack) -> Result<(), SymErr>>,
    simplifier: simplifier::Simplifier,
    debugging: bool,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            debugging: false,
        }
    }

    pub fn with_debugging(mut self) -> Self {
        self.debugging = true;
        self
    }

    pub fn with_functions(mut self) -> Self {
        functions::all(&mut self.functions);
        self
    }

    pub fn parse_infix(&self, infix_string: &str) -> Result<Expr, SymErr> {
        let infix = parse::parse_infix(infix_string)?;
        let postfix = parse::to_postfix(self, &infix)?;
        Ok(Expr {
            stack: postfix,
            engine: self,
        })
    }
}

impl<'a> std::fmt::Debug for Engine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("functions", self.functions.keys().borrow())
            .finish()
    }
}

#[derive(Debug)]
pub struct Expr<'a> {
    pub stack: Stack,
    engine: &'a Engine<'a>,
}

impl<'a> Expr<'a> {
    pub fn eval(&self) -> Result<Expr<'a>, SymErr> {
        let eval = eval::eval_postfix(self.engine, self.stack.clone(), false)?;
        Ok(Expr {
            stack: eval,
            engine: self.engine,
        })
    }

    pub fn evalf(&self) -> Result<Expr<'a>, SymErr> {
        let eval = eval::eval_postfix(self.engine, self.stack.clone(), true)?;
        Ok(Expr {
            stack: eval,
            engine: self.engine,
        })
    }

    pub fn print_infix(&self) -> Result<String, SymErr> {
        print_stack(&parse::to_infix(&self.stack)?, false)
    }

    pub fn print_postfix(&self) -> Result<String, SymErr> {
        print_stack(&self.stack, true)
    }

    pub fn print(&self) -> Result<String, SymErr> {
        self.print_infix()
    }
}

fn print_stack(stack: &Stack, delimiter: bool) -> Result<String, SymErr> {
    let mut output = String::new();
    for (i, symbol) in stack.iter().enumerate() {
        match symbol {
            Symbol::Operator(oper) => output.push(oper.to()),
            Symbol::Number(number) => output.push_str(number.to_string().as_str()),
            Symbol::String(str) => output.push_str(str.as_str()),
        }
        if delimiter && i != stack.len() - 1 {
            output.push(',')
        }
    }

    Ok(output)
}
