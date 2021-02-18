use std::collections::HashMap;

use super::{Stack, SymErr, Symbol};

type Function = fn(&mut Stack) -> Result<(), SymErr>;
type FunctionMap<'a> = HashMap<&'a str, Function>;

pub fn pop_n(input: &mut Stack, n: isize) -> Result<Vec<f64>, SymErr> {
    let mut output = Vec::new();

    for _ in 0..n {
        match input.pop().ok_or_else(|| return SymErr::StackEmpty)? {
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
        input.push(Symbol::String(String::from("ln")));
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
        input.push(Symbol::String(String::from("lon")));
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
        input.push(Symbol::String(String::from("sqrt")));
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
        input.push(Symbol::String(String::from("root")));
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
        input.push(Symbol::String(String::from("sin")));
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
        input.push(Symbol::String(String::from("cos")));
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
        input.push(Symbol::String(String::from("tan")));
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
        input.push(Symbol::String(String::from("asin")));
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
        input.push(Symbol::String(String::from("acos")));
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
        input.push(Symbol::String(String::from("atan")));
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
        input.push(Symbol::String(String::from("atan2")));
    }
    Ok(())
}

pub fn all(map: &mut FunctionMap) {
    map.insert("ln", ln);
    map.insert("log", log);
    map.insert("sqrt", sqrt);
    map.insert("root", root);
    map.insert("sin", sin);
    map.insert("cos", cos);
    map.insert("tan", tan);
    map.insert("asin", asin);
    map.insert("acos", acos);
    map.insert("atan", atan);
    map.insert("atan2", atan2);
}
