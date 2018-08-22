use ::lexer::*;

#[test]
fn can_lex_integers() {
    assert_eq!(Lexeme::Integer('5'), generate_lexeme_vector(&"5".to_string()).unwrap()[0])
}

#[test]
fn can_lex_chars() {
    assert_eq!(Lexeme::Char('c'), generate_lexeme_vector(&"c".to_string()).unwrap()[0])
}

#[test]
fn can_lex_whitespace() {
    assert_eq!(Lexeme::Whitespace, generate_lexeme_vector(&" ".to_string()).unwrap()[0])
}

#[test]
fn can_lex_left_paren() {
    assert_eq!(Lexeme::LeftParen, generate_lexeme_vector(&"(".to_string()).unwrap()[0])
}

#[test]
fn can_lex_right_paren() {
    assert_eq!(Lexeme::RightParen, generate_lexeme_vector(&")".to_string()).unwrap()[0])
}

#[test]
fn can_lex_colon() {
    assert_eq!(Lexeme::Colon, generate_lexeme_vector(&":".to_string()).unwrap()[0])
}

#[test]
fn can_lex_semicolon() {
    assert_eq!(Lexeme::Semicolon, generate_lexeme_vector(&";".to_string()).unwrap()[0])
}

#[test]
fn can_lex_period() {
    assert_eq!(Lexeme::Period, generate_lexeme_vector(&".".to_string()).unwrap()[0])
}

#[test]
fn can_lex_double_quote() {
    assert_eq!(Lexeme::DoubleQuote, generate_lexeme_vector(&"\"".to_string()).unwrap()[0])
}

#[test]
fn can_lex_pound() {
    assert_eq!(Lexeme::Pound, generate_lexeme_vector(&"#".to_string()).unwrap()[0])
}

#[test]
fn can_lex_left_bracket() {
    assert_eq!(Lexeme::LeftBracket, generate_lexeme_vector(&"[".to_string()).unwrap()[0])
}

#[test]
fn can_lex_right_bracket() {
    assert_eq!(Lexeme::RightBracket, generate_lexeme_vector(&"]".to_string()).unwrap()[0])
}

#[test]
fn invalid_chars_return_errors() {
    assert_eq!(Lexeme::Error('&'), generate_lexeme_vector(&"&".to_string()).unwrap()[0]);
    assert_eq!(Lexeme::Error('@'), generate_lexeme_vector(&"@".to_string()).unwrap()[0]);
    assert_eq!(Lexeme::Error('{'), generate_lexeme_vector(&"{".to_string()).unwrap()[0]);
}
