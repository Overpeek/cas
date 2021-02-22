use std::{borrow::Borrow, collections::HashMap, fmt::Display, ops};

use eval::eval_tree;
use simplifier::Simplifier;

pub mod constants;
pub mod eval;
pub mod functions;
pub mod parse;
pub mod simplifier;

type Stack = Vec<Symbol>;
type Negate = ();

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Number(f64),
    String(String),
    Operator(Operator),
    Negate(Negate),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Number(n) => write!(f, "{}", n),
            Symbol::String(s) => write!(f, "{}", s),
            Symbol::Operator(o) => write!(f, "{}", o.to()),
            Symbol::Negate(_) => write!(f, "{}", '-'),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Tree<T, U> {
    value: T,
    next: Option<(Box<U>, Box<U>)>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct List<T, U> {
    value: T,
    next: Option<Box<U>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Operator(Tree<Operator, Expr>),
    Negate(List<Negate, Expr>),
}

impl Expr {
    pub fn eval(&self) -> Self {
        eval_tree(&self)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "{}", s),
            Expr::Operator(o) => {
                write!(
                    f,
                    "{} -> [ {}, {} ]",
                    o.value.to(),
                    o.next.as_ref().unwrap().0,
                    o.next.as_ref().unwrap().1
                )
            }
            Expr::Negate(n) => write!(f, "-({})", n.next.as_ref().unwrap()),
        }
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Number(value)
    }
}

impl From<String> for Expr {
    fn from(value: String) -> Self {
        Expr::String(value)
    }
}

#[macro_export]
macro_rules! expr {
    ($e:expr) => {
        Expr::from($e)
    };
}

// ops

impl ops::Add for Expr {
    type Output = Expr;

    fn add(self, rhs: Self) -> Expr {
        Expr::Operator(Tree {
            value: Operator::Add,
            next: Some((Box::new(self), Box::new(rhs))),
        })
    }
}

impl ops::Sub for Expr {
    type Output = Expr;

    fn sub(self, rhs: Self) -> Expr {
        Expr::Operator(Tree {
            value: Operator::Sub,
            next: Some((Box::new(self), Box::new(rhs))),
        })
    }
}

impl ops::Neg for Expr {
    type Output = Expr;

    fn neg(self) -> Expr {
        Expr::Negate(List {
            value: (),
            next: Some(Box::new(self)),
        })
    }
}

impl ops::Mul for Expr {
    type Output = Expr;

    fn mul(self, rhs: Self) -> Expr {
        Expr::Operator(Tree {
            value: Operator::Mul,
            next: Some((Box::new(self), Box::new(rhs))),
        })
    }
}

impl ops::Div for Expr {
    type Output = Expr;

    fn div(self, rhs: Self) -> Expr {
        Expr::Operator(Tree {
            value: Operator::Div,
            next: Some((Box::new(self), Box::new(rhs))),
        })
    }
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
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Pow, // ^ (TODO: or **)
    LPa, // (
    RPa, // )
    Custom(u8, Associativity),
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
            Operator::Pow => Ok(5),
            Operator::Custom(p, _) => Ok(p.clone()),
            _ => Err(SymErr::InvalidOP),
        }
    }

    pub fn associativity(&self) -> Result<Associativity, SymErr> {
        match self {
            Operator::LPa | Operator::RPa => Err(SymErr::InvalidOP),
            Operator::Pow => Ok(Associativity::Right),
            Operator::Custom(_, a) => Ok(a.clone()),
            _ => Ok(Associativity::Left),
        }
    }

    pub fn to(&self) -> char {
        match self {
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Div => '/',
            Operator::Mul => '*',
            Operator::Pow => '^',
            Operator::LPa => '(',
            Operator::RPa => ')',
            Operator::Custom(_, _) => 'c',
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
            simplifier: Simplifier::new(),
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

    pub fn parse(&self, infix_string: &str) -> Result<Expr, SymErr> {
        Ok(parse::postfix_to_tree(&parse::to_postfix(
            self,
            &parse::parse_infix(infix_string)?,
        )?)?)
    }
}

impl<'a> std::fmt::Debug for Engine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("functions", self.functions.keys().borrow())
            .finish()
    }
}
