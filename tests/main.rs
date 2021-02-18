use std::{
    str::from_utf8,
    time::{SystemTime, UNIX_EPOCH},
};

use cas::{self, Engine, Symbol};

#[test]
fn simple_eval() {
    let engine = Engine::new().with_functions().with_debugging();

    let l: Vec<(&str, Symbol)> = vec![
        ("-3(-2^2*2)/2-0", Symbol::Number(12.0)),
        ("43*(4(2/5))", Symbol::Number(68.8)),
        ("5*4+3", Symbol::Number(23.0)),
        ("-2^6", Symbol::Number(-64.0)),
    ];

    for (i, e) in l.iter().enumerate() {
        let answer = engine
            .parse_infix(e.0)
            .unwrap()
            .evalf()
            .unwrap()
            .stack
            .first()
            .unwrap()
            .clone();

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
        for i in 1..(LEN + 1) {
            if bad_random() > 50 {
                count += 1;
                buf.push('-');
            } else {
                buf.push('+');
            };
        }
        buf.push('1');

        let answer = engine
            .parse_infix(buf.as_str())
            .unwrap()
            .eval()
            .unwrap()
            .stack
            .first()
            .unwrap()
            .clone();

        assert_eq!(
            answer,
            if count % 2 == 0 {
                Symbol::Number(2.0)
            } else {
                Symbol::Number(0.0)
            }
        );
    }
}
