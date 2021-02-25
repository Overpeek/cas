use super::{Expr, Operator};

pub fn eval_tree(tree: &Expr) -> Expr {
    match &tree {
        Expr::Operator(o) => match o.value {
            Operator::Pos => {
                let value = eval_tree(o.next.as_ref().unwrap()[0].as_ref());
                match value {
                    Expr::Number(n) => Expr::from(n),
                    _ => tree.clone(),
                }
            }
            Operator::Neg => {
                let value = eval_tree(o.next.as_ref().unwrap()[0].as_ref());
                match value {
                    Expr::Number(n) => Expr::from(-n),
                    _ => tree.clone(),
                }
            }
            _ => {
                let left = eval_tree(o.next.as_ref().unwrap()[0].as_ref());
                let right = eval_tree(o.next.as_ref().unwrap()[1].as_ref());

                match (&o.value, left, right) {
                    (Operator::Add, Expr::Number(number_left), Expr::Number(number_right)) => {
                        Expr::from(number_left + number_right)
                    }
                    (Operator::Sub, Expr::Number(number_left), Expr::Number(number_right)) => {
                        Expr::from(number_left - number_right)
                    }
                    (Operator::Mul, Expr::Number(number_left), Expr::Number(number_right)) => {
                        Expr::from(number_left * number_right)
                    }
                    (Operator::Div, Expr::Number(number_left), Expr::Number(number_right)) => {
                        Expr::from(number_left / number_right)
                    }
                    (Operator::Pow, Expr::Number(number_left), Expr::Number(number_right)) => {
                        Expr::from(number_left.powf(number_right))
                    }
                    _ => tree.clone(),
                }
            }
        },
        _ => tree.clone(),
    }
}
