use super::*;

#[test]
fn can_lex_integers() {
    assert_eq!(Lexeme::Integer(5), lex("5").unwrap()[0])
}

#[test]
fn can_lex_chars() {
    assert_eq!(Lexeme::Char('c'), lex("c").unwrap()[0])
}

#[test]
fn can_lex_left_paren() {
    assert_eq!(Lexeme::LeftParen, lex("(").unwrap()[0])
}

#[test]
fn can_lex_right_paren() {
    assert_eq!(Lexeme::RightParen, lex(")").unwrap()[0])
}

#[test]
fn can_lex_colon() {
    assert_eq!(Lexeme::Colon, lex(":").unwrap()[0])
}

#[test]
fn can_lex_semicolon() {
    assert_eq!(Lexeme::Semicolon, lex(";").unwrap()[0])
}

#[test]
fn can_lex_period() {
    assert_eq!(Lexeme::Period, lex(".").unwrap()[0])
}
