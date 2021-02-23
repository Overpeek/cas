use std::collections::HashMap;

use rand::random;

use super::{Stack, SymErr, Symbol};

type Function = fn(&mut Stack) -> Result<(), SymErr>;
type FunctionMap<'a> = HashMap<&'a str, (u8, Function)>;

pub fn pop_n(input: &mut Stack, n: isize) -> Result<Vec<f64>, SymErr> {
    let mut output = Vec::new();

    for _ in 0..n {
        match input.pop().ok_or(SymErr::StackEmpty)? {
            Symbol::Number(number) => output.push(number),
            _ => (),
        }
    }

    Ok(output)
}

// ln(x)
pub fn ln(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 1)?;
    if values.len() == 1 {
        input.push(Symbol::Number(values[0].ln()));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Function(String::from("ln")));
    }
    Ok(())
}

// log(base, x)
pub fn log(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 2)?;
    if values.len() == 2 {
        input.push(Symbol::Number(values[0].log(values[1])));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Number(values[1]));
        input.push(Symbol::Function(String::from("lon")));
    }
    Ok(())
}

// sqrt
pub fn sqrt(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 1)?;
    if values.len() == 1 {
        input.push(Symbol::Number(values[0].sqrt()));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Function(String::from("sqrt")));
    }
    Ok(())
}

// root
pub fn root(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 2)?;
    if values.len() == 2 {
        input.push(Symbol::Number(values[0].powf(1.0 / values[1])));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Number(values[1]));
        input.push(Symbol::Function(String::from("root")));
    }
    Ok(())
}

// sin(x)
pub fn sin(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 1)?;
    if values.len() == 1 {
        input.push(Symbol::Number(values[0].sin()));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Function(String::from("sin")));
    }
    Ok(())
}

// cos(x)
pub fn cos(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 1)?;
    if values.len() == 1 {
        input.push(Symbol::Number(values[0].cos()));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Function(String::from("cos")));
    }
    Ok(())
}

// tan(x)
pub fn tan(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 1)?;
    if values.len() == 1 {
        input.push(Symbol::Number(values[0].tan()));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Function(String::from("tan")));
    }
    Ok(())
}

// asin(x)
pub fn asin(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 1)?;
    if values.len() == 1 {
        input.push(Symbol::Number(values[0].asin()));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Function(String::from("asin")));
    }
    Ok(())
}

// acos(x)
pub fn acos(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 1)?;
    if values.len() == 1 {
        input.push(Symbol::Number(values[0].acos()));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Function(String::from("acos")));
    }
    Ok(())
}

// atan(x)
pub fn atan(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 1)?;
    if values.len() == 1 {
        input.push(Symbol::Number(values[0].atan()));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Function(String::from("atan")));
    }
    Ok(())
}

// atan2(x)
pub fn atan2(input: &mut Stack) -> Result<(), SymErr> {
    let values = pop_n(input, 2)?;
    if values.len() == 2 {
        input.push(Symbol::Number(values[0].atan2(values[1])));
    } else {
        input.push(Symbol::Number(values[0]));
        input.push(Symbol::Number(values[1]));
        input.push(Symbol::Function(String::from("atan2")));
    }
    Ok(())
}

// rand()
pub fn rand(input: &mut Stack) -> Result<(), SymErr> {
    input.push(Symbol::Number(random()));
    Ok(())
}

pub fn all(map: &mut FunctionMap) {
    map.insert("ln", (1, ln));
    map.insert("log", (2, log));
    map.insert("sqrt", (1, sqrt));
    map.insert("root", (2, root));
    map.insert("sin", (1, sin));
    map.insert("cos", (1, cos));
    map.insert("tan", (1, tan));
    map.insert("asin", (1, asin));
    map.insert("acos", (1, acos));
    map.insert("atan", (1, atan));
    map.insert("atan2", (2, atan2));
    map.insert("rand", (0, rand));
}
