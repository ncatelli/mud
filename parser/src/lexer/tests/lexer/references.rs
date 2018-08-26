use lexer::*;

#[test]
fn tokenize_valid_ref() {
    let t_vec = match lex("helloworld".to_string()) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    match first {
        Token::Ref(s) => match s.as_ref() {
            "helloworld" => (),
            _ => panic!("Token doesn't reference the correct object"),
        },
        _ => panic!("Token doesn't match Ref type."),
    };
}
