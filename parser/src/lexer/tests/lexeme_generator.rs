use lexer::*;

#[test]
fn can_lex_integers() {
    assert_eq!(
        Lexeme::Integer('5'),
        generate_lexeme_vector(&"5".to_string()).unwrap()[0]
    )
}

#[test]
fn can_lex_chars() {
    assert_eq!(
        Lexeme::Char('c'),
        generate_lexeme_vector(&"c".to_string()).unwrap()[0]
    )
}

#[test]
fn can_lex_whitespace() {
    assert_eq!(
        Lexeme::Whitespace,
        generate_lexeme_vector(&" ".to_string()).unwrap()[0]
    )
}

#[test]
fn can_lex_double_quote() {
    assert_eq!(
        Lexeme::DoubleQuote,
        generate_lexeme_vector(&"\"".to_string()).unwrap()[0]
    )
}

#[test]
fn can_lex_period() {
    assert_eq!(
        Lexeme::Period,
        generate_lexeme_vector(&".".to_string()).unwrap()[0]
    )
}

#[test]
fn invalid_chars_return_errors() {
    let invalid_chars = vec!['&', '@', '{', '}'];
    for c in invalid_chars.iter() {
        assert_eq!(
            Lexeme::Error(*c),
            generate_lexeme_vector(&c.to_string()).unwrap()[0]
        );
    }
}
