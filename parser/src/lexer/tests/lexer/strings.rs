use lexer::*;

#[test]
fn tokenize_valid_string() {
    let t = lex(&"\"hello\"".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    assert_eq!(Primitive::Str("hello".to_string()), *first);
}

#[test]
fn strings_respect_whitespace() {
    let t = lex(&"\"hello\" \"world\"".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    let second = &t_vec[1];
    assert_eq!(Primitive::Str("hello".to_string()), *first);
    assert_eq!(Primitive::Str("world".to_string()), *second);
}
