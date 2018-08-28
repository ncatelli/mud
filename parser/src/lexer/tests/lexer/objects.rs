use lexer::*;

#[test]
fn tokenize_valid_object() {
    let t = lex("#1337".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    match first {
        Token::Operator(o) => match o {
            Operator::Pound => (),
        },
        _ => panic!("Token doesn't match operand."),
    };

    let second = &t_vec[1];

    match second {
        Token::Operand(p) => match p {
            Primitive::Int(i) => match *i {
                1337 => (),
                _ => panic!("Object Id didn't parse correctly."),
            },
            _ => panic!("Primitive isn't an Int."),
        },
        _ => panic!("Token isn't an operand"),
    }
}
