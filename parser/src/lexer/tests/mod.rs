use super::*;

#[test]
fn can_lex_integers() {
    let lexeme = "5";
    assert_eq!(Lexeme::Integer(5), lex(lexeme).unwrap()[0])
}

#[test]
fn can_lex_chars() {
    let lexeme = "c";
    assert_eq!(Lexeme::Char('c'), lex(lexeme).unwrap()[0])
}
