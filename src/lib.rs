use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Display,
    ops::{self, Rem},
};

use eval::eval_tree;
use simplifier::Simplifier;

pub mod constants;
pub mod eval;
pub mod functions;
pub mod parse;
pub mod simplifier;

type Stack = Vec<Symbol>;

pub struct Engine<'a> {
    functions: HashMap<&'a str, (u8, fn(&mut Stack) -> Result<(), SymErr>)>,
    simplifier: simplifier::Simplifier,
    debugging: bool,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Rational(i64, i64),
    Irrational(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Number(Number),
    Variable(String),
    Function(String),
    Operator(Operator),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Tree<T, U> {
    value: T,
    next: Option<Vec<Box<U>>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Number(Number),
    Variable(String),
    Function(Tree<String, Expr>),
    Operator(Tree<Operator, Expr>),

    Identifier(u32),
}

impl Display for Number {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Irrational(n) => write!(fmt, "{}", n),
            Number::Rational(nom, denom) => {
                if *denom == 1 {
                    write!(fmt, "{}", nom)
                } else {
                    write!(fmt, "({}/{})", nom, denom)
                }
            }
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Number(n) => write!(fmt, "{}", n),
            Symbol::Variable(s) => write!(fmt, "{}", s),
            Symbol::Function(s) => write!(fmt, "{}()", s),
            Symbol::Operator(Operator::Neg) => write!(fmt, "{}", '-'),
            Symbol::Operator(o) => write!(fmt, "{}", o.to()),
        }
    }
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

    pub fn eval(&self, engine: &Engine) -> Self {
        eval_tree(engine, &self)
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
            Expr::Operator(o) => match o.value {
                Operator::Pos => format!("-({})", o.next.as_ref().unwrap()[0]),
                Operator::Neg => format!("-({})", o.next.as_ref().unwrap()[0]),
                _ => format!(
                    "{} -> [ {}, {} ]",
                    o.value.to(),
                    o.next.as_ref().unwrap()[0].print_debug(),
                    o.next.as_ref().unwrap()[1].print_debug()
                ),
            },
            Expr::Identifier(i) => format!("\\{}\\", i),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.print())
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::Number(Number::Rational(value, 1))
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Number(Number::Irrational(value))
    }
}

impl From<Number> for Expr {
    fn from(value: Number) -> Self {
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
        if let Expr::Number(l) = &self {
            if let Expr::Number(r) = &rhs {
                return Expr::Number(l.clone() + r.clone());
            }
        }

        Expr::Operator(Tree {
            value: Operator::Add,
            next: Some(vec![Box::new(self), Box::new(rhs)]),
        })
    }
}

impl ops::Sub for Expr {
    type Output = Expr;

    fn sub(self, rhs: Self) -> Expr {
        if let Expr::Number(l) = &self {
            if let Expr::Number(r) = &rhs {
                return Expr::Number(l.clone() - r.clone());
            }
        }

        Expr::Operator(Tree {
            value: Operator::Sub,
            next: Some(vec![Box::new(self), Box::new(rhs)]),
        })
    }
}

impl ops::Neg for Expr {
    type Output = Expr;

    fn neg(self) -> Expr {
        if let Expr::Number(l) = &self {
            return Expr::Number(-l.clone());
        }

        Expr::Operator(Tree {
            value: Operator::Neg,
            next: Some(vec![Box::new(self)]),
        })
    }
}

impl ops::Mul for Expr {
    type Output = Expr;

    fn mul(self, rhs: Self) -> Expr {
        if let Expr::Number(l) = &self {
            if let Expr::Number(r) = &rhs {
                return Expr::Number(l.clone() * r.clone());
            }
        }

        Expr::Operator(Tree {
            value: Operator::Mul,
            next: Some(vec![Box::new(self), Box::new(rhs)]),
        })
    }
}

impl ops::Div for Expr {
    type Output = Expr;

    fn div(self, rhs: Self) -> Expr {
        if let Expr::Number(l) = &self {
            if let Expr::Number(r) = &rhs {
                return Expr::Number(l.clone() / r.clone());
            }
        }

        Expr::Operator(Tree {
            value: Operator::Div,
            next: Some(vec![Box::new(self), Box::new(rhs)]),
        })
    }
}

impl From<Number> for f64 {
    fn from(n: Number) -> Self {
        match n {
            Number::Rational(nom, denom) => nom as f64 / denom as f64,
            Number::Irrational(n) => n,
        }
    }
}

impl ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Rational(left_nom, left_denom), Number::Rational(right_nom, right_denom)) => {
                let nom = left_nom * right_denom + right_nom * left_denom;
                let denom = left_denom * right_denom;
                let gcf = Number::gcf(nom, denom);
                Number::Rational(nom / gcf, denom / gcf)
            }
            (lhs, rhs) => {
                let lhs: f64 = lhs.into();
                let rhs: f64 = rhs.into();
                Number::Irrational(lhs + rhs)
            }
        }
    }
}

impl ops::Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Rational(left_nom, left_denom), Number::Rational(right_nom, right_denom)) => {
                let nom = left_nom * right_denom - right_nom * left_denom;
                let denom = left_denom * right_denom;
                let gcf = Number::gcf(nom, denom);
                Number::Rational(nom / gcf, denom / gcf)
            }
            (lhs, rhs) => {
                let lhs: f64 = lhs.into();
                let rhs: f64 = rhs.into();
                Number::Irrational(lhs - rhs)
            }
        }
    }
}

impl ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Rational(nom, denom) => Number::Rational(-nom, denom),
            Number::Irrational(f) => Number::Irrational(-f),
        }
    }
}

impl ops::Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Rational(left_nom, left_denom), Number::Rational(right_nom, right_denom)) => {
                let nom = left_nom * right_nom;
                let denom = left_denom * right_denom;
                let gcf = Number::gcf(nom, denom);
                Number::Rational(nom / gcf, denom / gcf)
            }
            (lhs, rhs) => {
                let lhs: f64 = lhs.into();
                let rhs: f64 = rhs.into();
                Number::Irrational(lhs * rhs)
            }
        }
    }
}

impl ops::Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Rational(left_nom, left_denom), Number::Rational(right_nom, right_denom)) => {
                let nom = left_nom * right_denom;
                let denom = left_denom * right_nom;
                let gcf = Number::gcf(nom, denom);
                Number::Rational(nom / gcf, denom / gcf)
            }
            (lhs, rhs) => {
                let lhs: f64 = lhs.into();
                let rhs: f64 = rhs.into();
                Number::Irrational(lhs / rhs)
            }
        }
    }
}

impl Number {
    pub fn parse(from: &str) -> Result<Self, SymErr> {
        if from.contains('.') {
            Ok(Number::Irrational(
                from.parse::<f64>().or(Err(SymErr::NotANumber))?,
            ))
        } else {
            Ok(Number::Rational(
                from.parse::<i64>().or(Err(SymErr::NotANumber))?,
                1,
            ))
        }
    }

    fn gcf(lhs: i64, rhs: i64) -> i64 {
        if rhs == 0 {
            lhs
        } else {
            Number::gcf(rhs, lhs.rem(rhs))
        }
    }
}

impl Expr {
    pub fn operate(self, oper: Operator, rhs: Option<Self>) -> Result<Self, SymErr> {
        match oper {
            Operator::Pos => Ok(self),
            Operator::Neg => Ok(-self),
            Operator::Add => Ok(self + rhs.ok_or(SymErr::InvalidFunctionArgCount)?),
            Operator::Sub => Ok(self - rhs.ok_or(SymErr::InvalidFunctionArgCount)?),
            Operator::Mul => Ok(self * rhs.ok_or(SymErr::InvalidFunctionArgCount)?),
            Operator::Div => Ok(self / rhs.ok_or(SymErr::InvalidFunctionArgCount)?),
            Operator::Pow => Ok(self.pow(rhs.ok_or(SymErr::InvalidFunctionArgCount)?)),
            _ => Err(SymErr::InvalidOP),
        }
    }

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
            Expr::Number(_) => 2,
            Expr::Operator(_) => 3,
            Expr::Variable(_) => 4,
        }
    }
}

impl Operator {
    pub fn is_parenthesis(&self) -> bool {
        *self == Operator::LPa || *self == Operator::RPa
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
            Operator::Pos => 'p',
            Operator::Neg => 'n',
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Div => '/',
            Operator::Mul => '*',
            Operator::Pow => '^',
            Operator::LPa => '(',
            Operator::RPa => ')',
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
