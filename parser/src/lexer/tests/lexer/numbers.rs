use lexer::*;

#[test]
fn tokenize_valid_integer() {
    let t = lex("1337".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    match first {
        Token::Operand(o) => match o {
            Primitive::Int(i) => {
                if *i != 1337 {
                    panic!("Token integer not parsed correctly")
                }
            }
            _ => panic!("token isn't an Integer."),
        },
        _ => panic!("Token doesn't match operand."),
    };
}
