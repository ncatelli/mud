use lexer::*;

#[test]
fn tokenize_valid_integer() {
    let t = lex(&"1337".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    assert_eq!(Primitive::Int(1337), *first);
}

#[test]
fn integers_respect_whitespace() {
    let t = lex(&"1337 42".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    let second = &t_vec[1];
    assert_eq!(Primitive::Int(1337), *first);
    assert_eq!(Primitive::Int(42), *second);
}

#[test]
fn tokenize_valid_float() {
    let t = lex(&"3.14".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    assert_eq!(Primitive::Float(3.14), *first);
}

#[test]
fn floats_respect_whitespace() {
    let t = lex(&"3.14 4.2".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    let second = &t_vec[1];
    assert_eq!(Primitive::Float(3.14), *first);
    assert_eq!(Primitive::Float(4.2), *second);
}
