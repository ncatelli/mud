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

#[test]
fn can_lex_single_quote() {
    assert_eq!(Lexeme::SingleQuote, lex("'").unwrap()[0])
}

#[test]
fn can_lex_double_quote() {
    assert_eq!(Lexeme::DoubleQuote, lex("\"").unwrap()[0])
}

#[test]
fn can_lex_pound() {
    assert_eq!(Lexeme::Pound, lex("#").unwrap()[0])
}

#[test]
fn invalid_chars_return_errors() {
    assert_eq!(Lexeme::Error('&'), lex("&").unwrap()[0]);
    assert_eq!(Lexeme::Error('@'), lex("@").unwrap()[0]);
    assert_eq!(Lexeme::Error('{'), lex("{").unwrap()[0]);
}
