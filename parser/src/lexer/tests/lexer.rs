use ::lexer::*;

#[test]
#[ignore]
fn it_works() {
    let t = lex("hello".to_string());
    let t_vec = t.expect("lexer failed to parse token");
    
    let first = &t_vec[0];
    let _stok = match first {
        Token::Operand(o) => match o {
            Primitive::Str(s) => s,
            _ => panic!("token isn't a string"),
        },
        _ => panic!("Token doesn't match operand."),
     };
}
