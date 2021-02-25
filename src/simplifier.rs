use std::collections::HashMap;

use crate::{expr, Expr, Engine};

pub struct Simplifier {
    rules: Vec<(Expr, Expr)>,
}

impl Simplifier {

	fn compare(matcher: &Expr, target: &Expr, ids: &mut HashMap<u32, Expr>) -> bool {
		if matcher.ty() != target.ty() {
			return false;
		}
		
		match (matcher, target) {
			(Expr::Number(a), Expr::Number(b)) => a == b,
			(Expr::Operator(a), Expr::Operator(b)) => {
				if a.value != b.value {
					return false;
				}

				let matcher_left = a.next.as_ref().unwrap()[0].as_ref();
				let matcher_right = a.next.as_ref().unwrap()[1].as_ref();
				let target_left = b.next.as_ref().unwrap()[0].as_ref();
				let target_right = b.next.as_ref().unwrap()[1].as_ref();

				let left_match = if let Expr::Identifier(i) = matcher_left {
					if let Some(e) = ids.get(i) {
						e == target_left
					} else {
						ids.insert(i.clone(), target_left.clone());
						true
					}
				} else {
					Simplifier::compare(matcher_left, target_left, ids)
				};
				let right_match = if let Expr::Identifier(i) = matcher_right {
					if let Some(e) = ids.get(i) {
						e == target_right
					} else {
						ids.insert(i.clone(), target_right.clone());
						true
					}
				} else {
					Simplifier::compare(matcher_right, target_right, ids)
				};

				left_match && right_match
			},
			_ => panic!("No match for matcher and target"),
		}
	}

	fn replace(expr: &mut Expr, ids: &mut HashMap<u32, Expr>) {
		match expr {
			Expr::Identifier(i) => {
				if let Some(e) = ids.get(i) {
					*expr = e.clone();
					return;
				}
			}
			Expr::Function(f) => {
				f.next.as_mut().unwrap().iter_mut().for_each(|e| Simplifier::replace(e.as_mut(), ids));
			}
			Expr::Operator(o) => {
				o.next.as_mut().unwrap().iter_mut().for_each(|e| Simplifier::replace(e.as_mut(), ids));
			}
			Expr::Negate(n) => {
				n.next.as_mut().unwrap().iter_mut().for_each(|e| Simplifier::replace(e.as_mut(), ids));
			}
			_ => (),
		}
	}

	pub fn simplify_r(&self, engine: &Engine, expr: &Expr) -> (Expr, bool) {
		let mut simplified = expr.clone();
		let mut found = false;

		// apply all rules
		for (matcher, replace) in self.rules.iter() {
			// simplify root
			let mut ids = HashMap::new();
			if Simplifier::compare(matcher, &simplified, &mut ids) {
				simplified = replace.clone();
				Simplifier::replace(&mut simplified, &mut ids);
				
				if engine.debugging {
					println!("Match found: {} for {} and replaced with {}", matcher, expr, simplified);
				}

				found = true;
			}
		}

        (simplified, found)
	}

    pub fn simplify(&self, engine: &Engine, expr: &Expr) -> Expr {
		let mut expr = expr.clone();
		loop {
			let res  = self.simplify_r(engine, &expr);
			expr = res.0;

			if !res.1 {
				break;
			}
		}

		expr
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new() -> Self {
        let mut rules = Vec::<(Expr, Expr)>::new();

		// ordering

        rules.push((
			expr!(0) + (expr!(1) + expr!(2)),
			(expr!(0) + expr!(1)) + expr!(2)
		));

		// simple canceling out

        // x + 0 = x
        rules.push((
			expr!(0.0) + expr!(0),
			expr!(0)
		));
		// 0 + x = x
        rules.push((
			expr!(0) + expr!(0.0),
			expr!(0)
		));
		// x - x = 0
        rules.push((
			expr!(0) - expr!(0),
			expr!(0.0)
		));
        // x * 0 = 0
        rules.push((
			expr!(0.0) * expr!(0),
			expr!(0.0)
		));
		// 0 * x = 0
        rules.push((
			expr!(0) * expr!(0.0),
			expr!(0.0)
		));
        // x * 1 = x
        rules.push((
			expr!(1.0) * expr!(0),
			expr!(0)
		));
		// 1 * x = x
        rules.push((
			expr!(0) * expr!(1.0),
			expr!(0)
		));



        // x * x = x^2
        rules.push((
			expr!(0) * expr!(0),
			expr!(0).pow(expr!(2.0))
		));
        // x + x = 2x
        rules.push((
			expr!(0) + expr!(0),
			expr!(2.0) * expr!(0)
		));
        // x / x = 1
        rules.push((
			expr!(0) / expr!(0),
			expr!(1.0)
		));
        // x * x^y = x^(y+1)
        rules.push((
			expr!(0) * expr!(0).pow(expr!(1)),
			expr!(0).pow(expr!(1) + expr!(1.0))
		));
        // x^y * x = x^(y+1)
        rules.push((
			expr!(0).pow(expr!(1)) * expr!(0),
			expr!(0).pow(expr!(1) + expr!(1.0))
		));
        // (x^y)^z = x^(y*z)
        rules.push((
			expr!(0).pow(expr!(1)).pow(expr!(2)),
			expr!(0).pow(expr!(1) * expr!(2))
		));
        // x * x^-1 = 1
        rules.push((
			expr!(0) * expr!(0).pow(expr!(-1.0)),
			expr!(1.0)
		));
        // x / y = x * y^-1
        rules.push((
			expr!(0) / expr!(1), 
			expr!(0) * expr!(1).pow(expr!(-1.0))
		));

		// x + x * y

        // x + x * y = x * (y + 1)
        rules.push((
            expr!(0) + expr!(0) * expr!(1),
            expr!(0) * (expr!(1) + expr!(1.0)),
        ));
        // x + y * x = x * (y + 1)
        rules.push((
            expr!(0) + expr!(1) * expr!(0),
            expr!(0) * (expr!(1) + expr!(1.0)),
        ));
        // x * y + x = x * (y + 1)
        rules.push((
            expr!(0) * expr!(1) + expr!(0),
            expr!(0) * (expr!(1) + expr!(1.0)),
        ));
        // y * x + x = x * (y + 1)
        rules.push((
            expr!(1) * expr!(0) + expr!(0),
            expr!(0) * (expr!(1) + expr!(1.0)),
        ));
		// x - x * y
		// x - x * y = x * (y - 1)
        rules.push((
            expr!(0) - expr!(0) * expr!(1),
            expr!(0) * (expr!(1) - expr!(1.0)),
		));
		// x * y - x = x * (y - 1)
        rules.push((
            expr!(0) * expr!(1) - expr!(0),
            expr!(0) * (expr!(1) - expr!(1.0)),
		));
		// x - y * x = x * (y - 1)
        rules.push((
            expr!(0) - expr!(1) * expr!(0),
            expr!(0) * (expr!(1) - expr!(1.0)),
		));
		// y * x - x = x * (y - 1)
        rules.push((
            expr!(1) * expr!(0) - expr!(0),
            expr!(0) * (expr!(1) - expr!(1.0)),
		));

		// some power rules

        // x^y * x^z = x^(y+z)
        rules.push((
            expr!(0).pow(expr!(1)) * expr!(0).pow(expr!(2)),
            expr!(0).pow(expr!(1) + expr!(2)),
        ));

		// functions

        // sin(x)^2 + cos(x)^2 = 1
        rules.push((
            Expr::function("sin", vec![expr!(0)]).pow(expr!(2.0)) + Expr::function("cos", vec![expr!(0)]).pow(expr!(2.0)),
            expr!(1.0),
        ));
        // log(e, x) = ln(x)
        rules.push((
            Expr::function("log", vec![expr!("e"), expr!(0)]),
            Expr::function("ln", vec![expr!(0)]),
        ));

        Simplifier { rules }
    }
}
