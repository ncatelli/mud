use lexer::*;

#[test]
fn tokenize_valid_symbol() {
    let t_vec = match lex(&"helloworld".to_string()) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    match first {
        Primitive::Symbol(s) => match s.as_ref() {
            "helloworld" => (),
            _ => panic!("Token doesn't reference the correct symbol"),
        },
        _ => panic!("Token is not a Symbol.")
    };
}
