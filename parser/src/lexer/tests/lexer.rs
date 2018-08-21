use ::lexer::*;

#[test]
#[ignore]
fn it_works() {
    let t = lex("hello");

    let stok = match t.expect("lexer failed to parse token")[0] {
        Token::Operand(o) => match o {
            ComplexType::Primitive(p) => match p {
                Primitive::Str(s) => s,
                _ => panic!("token isn't a string"),
            },
            _ => panic!("Operand is not a primitive.")
        },
        _ => panic!("Token doesn't match operand."),
     };
}
