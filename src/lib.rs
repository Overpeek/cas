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
    Variable(String),
    Function(String),
    Operator(Operator),
    Negate(Negate),
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Number(n) => write!(fmt, "{}", n),
            Symbol::Variable(s) => write!(fmt, "{}", s),
            Symbol::Function(s) => write!(fmt, "{}()", s),
            Symbol::Operator(o) => write!(fmt, "{}", o.to()),
            Symbol::Negate(_) => write!(fmt, "{}", '-'),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Tree<T, U> {
    value: T,
    next: Option<Vec<Box<U>>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Function(Tree<String, Expr>),
    Operator(Tree<Operator, Expr>),
    Negate(Tree<Negate, Expr>),

    Identifier(u32),
}

impl Expr {
    pub fn parse(engine: &Engine, infix_string: &str) -> Result<Expr, SymErr> {
        Ok(parse::postfix_to_tree(
            &engine,
            &parse::to_postfix(engine, &parse::parse_infix(engine, infix_string)?)?,
        )?)
    }

    pub fn simplify(&self, engine: &Engine) -> Expr {
        engine.simplifier.simplify(engine, &self)
    }

    pub fn eval(&self) -> Self {
        eval_tree(&self)
    }

    pub fn print(&self) -> String {
        parse::tree_to_infix(&self)
    }

    pub fn print_latex(&self) -> String {
        parse::tree_to_latex(&self)
    }

    pub fn print_debug(&self) -> String {
        match &self {
            Expr::Number(n) => format!("{}", n),
            Expr::Variable(s) => format!("{}", s),
            Expr::Function(f) => {
                let mut l = String::new();
                f.next.as_ref().unwrap().iter().for_each(|e| {
                    l = format!("{}, {}", l, e.print_debug());
                });

                format!(
                    "{}({})",
                    f.value,
                    if l.is_empty() { l.as_str() } else { &l[2..] },
                )
            }
            Expr::Operator(o) => {
                format!(
                    "{} -> [ {}, {} ]",
                    o.value.to(),
                    o.next.as_ref().unwrap()[0].print_debug(),
                    o.next.as_ref().unwrap()[1].print_debug()
                )
            }
            Expr::Negate(n) => format!("-({})", n.next.as_ref().unwrap()[0]),
            Expr::Identifier(i) => format!("\\{}\\", i),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.print())
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Number(value)
    }
}

impl From<String> for Expr {
    fn from(value: String) -> Self {
        Expr::Variable(value)
    }
}

impl From<&str> for Expr {
    fn from(value: &str) -> Self {
        Expr::Variable(value.into())
    }
}

impl From<u32> for Expr {
    fn from(value: u32) -> Self {
        Expr::Identifier(value)
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
            next: Some(vec![Box::new(self), Box::new(rhs)]),
        })
    }
}

impl ops::Sub for Expr {
    type Output = Expr;

    fn sub(self, rhs: Self) -> Expr {
        Expr::Operator(Tree {
            value: Operator::Sub,
            next: Some(vec![Box::new(self), Box::new(rhs)]),
        })
    }
}

impl ops::Neg for Expr {
    type Output = Expr;

    fn neg(self) -> Expr {
        Expr::Negate(Tree {
            value: (),
            next: Some(vec![Box::new(self)]),
        })
    }
}

impl ops::Mul for Expr {
    type Output = Expr;

    fn mul(self, rhs: Self) -> Expr {
        Expr::Operator(Tree {
            value: Operator::Mul,
            next: Some(vec![Box::new(self), Box::new(rhs)]),
        })
    }
}

impl ops::Div for Expr {
    type Output = Expr;

    fn div(self, rhs: Self) -> Expr {
        Expr::Operator(Tree {
            value: Operator::Div,
            next: Some(vec![Box::new(self), Box::new(rhs)]),
        })
    }
}

impl Expr {
    pub fn pow(self, exp: Self) -> Expr {
        Expr::Operator(Tree {
            value: Operator::Pow,
            next: Some(vec![Box::new(self), Box::new(exp)]),
        })
    }

    pub fn function<S>(name: S, exp: Vec<Self>) -> Expr
    where
        S: Into<String>,
    {
        Expr::Function(Tree {
            value: name.into(),
            next: Some(exp.into_iter().map(|e| Box::new(e)).collect::<Vec<_>>()),
        })
    }

    fn ty(&self) -> i64 {
        match self {
            Expr::Function(_) => 0,
            Expr::Identifier(_) => 1,
            Expr::Negate(_) => 2,
            Expr::Number(_) => 3,
            Expr::Operator(_) => 4,
            Expr::Variable(_) => 5,
        }
    }
}

#[derive(Debug)]
pub enum SymErr {
    StackEmpty,
    NotANumber,
    InvalidOP,
    InvalidSign,
    UnknownFunction,
    InvalidFunctionArgCount,
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
    functions: HashMap<&'a str, (u8, fn(&mut Stack) -> Result<(), SymErr>)>,
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
}

impl<'a> std::fmt::Debug for Engine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("functions", self.functions.keys().borrow())
            .finish()
    }
}
