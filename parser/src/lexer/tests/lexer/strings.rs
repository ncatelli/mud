use lexer::*;

#[test]
fn tokenize_valid_string() {
    let t = lex("\"hello\"".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    match first {
        Token::Operand(o) => match o {
            Primitive::Str(s) => s,
            _ => panic!("token isn't a string"),
        },
    };
}
