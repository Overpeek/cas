use std::time::{SystemTime, UNIX_EPOCH};

use cas::{self, expr, Engine, Expr};

#[test]
fn simple_eval() {
    let engine = Engine::new().with_functions().with_debugging();

    let l: Vec<(&str, Expr)> = vec![
        ("-3(-2^2*2)/2-0", expr!(12.0)),
        ("43*(4(2/5))", expr!(68.8)),
        ("5*4+3", expr!(23.0)),
        ("-2^6", expr!(-64.0)),
    ];

    for (i, e) in l.iter().enumerate() {
        let answer = Expr::parse(&engine, e.0).unwrap().eval();

        assert_eq!(answer, e.1, "e={}, i={}", e.0, i);
    }
}

#[test]
fn eval_after_simplify() {
    let engine = Engine::new().with_functions().with_debugging();

    let l: Vec<(&str, Expr)> = vec![
        ("-3(-2^2*2)/2-0", expr!(12.0)),
        ("43*(4(2/5))", expr!(68.8)),
        ("5*4+3", expr!(23.0)),
        ("-2^6", expr!(-64.0)),
    ];

    for (i, e) in l.iter().enumerate() {
        let answer = Expr::parse(&engine, e.0).unwrap().simplify(&engine).eval();

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

    const LEN: usize = 100;
    let mut buf = String::new();
    buf.reserve(LEN);

    for _ in 0..100 {
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

        let answer = Expr::parse(&engine, buf.as_str()).unwrap().eval();

        assert_eq!(
            answer,
            if count % 2 == 0 {
                expr!(2.0)
            } else {
                expr!(0.0)
            }
        );
    }
}
