use std::time::{SystemTime, UNIX_EPOCH};

use cas::{self, expr, Engine, Expr, Number, SymErr};

#[test]
fn simple_eval() {
    let engine = Engine::new().with_functions().with_debugging();

    let l: Vec<(&str, Expr)> = vec![
        ("-3(-2^2*2)/2-0", expr!(12)),
        ("43*(4(81/9))", expr!(1548)),
        ("5*4+3", expr!(23)),
        ("-2^6", expr!(-64)),
    ];

    for (i, e) in l.iter().enumerate() {
        let answer = Expr::parse(&engine, e.0).unwrap().eval(&engine).unwrap();

        assert_eq!(answer, e.1, "e={}, i={}", e.0, i);
    }
}

#[test]
fn eval_after_simplify() {
    let engine = Engine::new().with_functions().with_debugging();

    let l: Vec<(&str, Expr)> = vec![
        ("-3(-2^2*2)/2-0", expr!(12)),
        ("43*(4(81/9))", expr!(1548)),
        ("5*4+3", expr!(23)),
        ("-2^6", expr!(-64)),
    ];

    for (i, e) in l.iter().enumerate() {
        let answer = Expr::parse(&engine, e.0)
            .unwrap()
            .simplify(&engine)
            .eval(&engine)
            .unwrap();

        assert_eq!(answer, e.1, "e={}, i={}", e.0, i);
    }
}

#[test]
fn sign_fuzz() {
    let engine = Engine::new().with_functions().with_debugging();

    let mut nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
        % 100;
    let mut bad_random = || {
        nanos = (nanos * 809 + 971) % 100;
        nanos
    };

    const LEN: usize = 25;
    let mut buf = String::new();
    buf.reserve(LEN);

    for _ in 0..200 {
        let mut count = 0;
        buf.clear();
        buf.push('1');
        for _ in 1..(LEN + 1) {
            if bad_random() > 50 {
                count += 1;
                buf.push('-');
            } else {
                buf.push('+');
            };
        }
        buf.push('1');

        let answer = Expr::parse(&engine, buf.as_str())
            .unwrap()
            .eval(&engine)
            .unwrap();

        assert_eq!(
            answer,
            if count % 2 == 0 {
                expr!(2)
            } else {
                expr!(0)
            }
        );
    }
}

#[test]
fn number_test() {
    // irrationals
    assert_eq!(
        Number::Irrational(1.0) + Number::Irrational(1.0),
        Number::Irrational(2.0)
    );
    assert_eq!(
        Number::Irrational(1.0) - Number::Irrational(1.0),
        Number::Irrational(0.0)
    );
    assert_eq!(
        Number::Irrational(1.0) * Number::Irrational(1.0),
        Number::Irrational(1.0)
    );
    assert_eq!(
        Number::Irrational(1.0) / Number::Irrational(1.0),
        Number::Irrational(1.0)
    );
    // mix
    assert_eq!(
        Number::Rational(1, 1) + Number::Irrational(1.0),
        Number::Irrational(2.0)
    );
    assert_eq!(
        Number::Rational(1, 1) - Number::Irrational(1.0),
        Number::Irrational(0.0)
    );
    assert_eq!(
        Number::Rational(1, 1) * Number::Irrational(1.0),
        Number::Irrational(1.0)
    );
    assert_eq!(
        Number::Rational(1, 1) / Number::Irrational(1.0),
        Number::Irrational(1.0)
    );
    // ratonals
    assert_eq!(
        Number::Rational(1, 2) + Number::Rational(1, 2),
        Number::Rational(1, 1)
    );
    assert_eq!(
        Number::Rational(8, 4) - Number::Rational(4, 2),
        Number::Rational(0, 1)
    );
    assert_eq!(
        Number::Rational(4, 1) * Number::Rational(1, 2),
        Number::Rational(2, 1)
    );
    assert_eq!(
        Number::Rational(4, 1) / Number::Rational(2, 1),
        Number::Rational(2, 1)
    );
    // parse
    assert_eq!(Number::parse("5.0").unwrap(), Number::Irrational(5.0));
    assert_eq!(Number::parse("50").unwrap(), Number::Rational(50, 1));
    assert_eq!(Number::parse("5.5.0").unwrap_err(), SymErr::NotANumber);
}
