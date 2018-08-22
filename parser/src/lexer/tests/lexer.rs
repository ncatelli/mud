use ::lexer::*;

#[test]
#[ignore]
fn it_works() {
    let t = lex("hello");

    let stok = match t.expect("lexer failed to parse token")[0] {
        Token::Operand(o) => match o {
            Primitive::Str(s) => s,
            _ => panic!("token isn't a string"),
        },
        _ => panic!("Token doesn't match operand."),
     };
}
