extern crate parser;

use parser::lexer::Primitive;

#[test]
fn parse_returns_ok_on_valid_statment() {
    let full_sentence = "take pencil from table";
    match parser::parse(full_sentence.to_string()) {
        Ok(c) => {
            assert_eq!(Some(Primitive::Symbol("take".to_string())), c.verb());
            assert_eq!(
                Some(Primitive::Symbol("pencil".to_string())),
                c.direct_object()
            );
            assert_eq!(Some(Primitive::Symbol("from".to_string())), c.preposition());
            assert_eq!(
                Some(Primitive::Symbol("table".to_string())),
                c.indirect_object()
            );
        }
        Err(_) => panic!("Parse(\"{}\") returns an Err.", full_sentence),
    }
}
