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
        Token::Operand(o) => match o {
            Primitive::Object(i) => {
                if *i != 1337 {
                    panic!("Token id not parsed correctly")
                }
            }
            _ => panic!("token isn't an Object."),
        },
        _ => panic!("Token doesn't match operand."),
    };
}
