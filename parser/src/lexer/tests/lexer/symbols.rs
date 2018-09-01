use lexer::*;

#[test]
fn tokenize_valid_symbol() {
    let t_vec = match lex(&"helloworld".to_string()) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    assert_eq!(Primitive::Symbol("helloworld".to_string()), *first);
}

#[test]
fn symbols_respect_whitespace() {
    let t_vec = match lex(&"hello world".to_string()) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    let second = &t_vec[1];

    assert_eq!(Primitive::Symbol("hello".to_string()), *first);

    assert_eq!(Primitive::Symbol("world".to_string()), *second);
}
