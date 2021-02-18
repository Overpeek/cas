use std::{borrow::Borrow, collections::HashMap};

mod constants;
mod eval;
mod functions;
mod parse;

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
    ParenthesesMismatch,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
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

    pub fn precedence(&self) -> Result<u8, SymErr> {
        match self {
            Operator::Add => Ok(2),
            Operator::Sub => Ok(2),
            Operator::Div => Ok(3),
            Operator::Mul => Ok(3),
            Operator::Pow => Ok(4),
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
        let infix = parse::parse_infix(infix_string);
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
}
