use lexer::*;

#[test]
fn tokenize_valid_integer() {
    let t = lex(&"1337".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    match first {
        Primitive::Int(i) => {
            if *i != 1337 {
                panic!("Token integer not parsed correctly")
            }
        }
        _ => panic!("token isn't an Integer."),
    };
}

#[test]
fn tokenize_valid_float() {
    let t = lex(&"3.14".to_string());
    let t_vec = match t {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let first = &t_vec[0];
    match first {
        Primitive::Float(i) => {
            if *i != 3.14 {
                panic!("Token float not parsed correctly")
            }
        }
        _ => panic!("token isn't an Float."),
    };
}
