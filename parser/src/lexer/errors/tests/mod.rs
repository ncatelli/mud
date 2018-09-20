use lexer::errors::*;

#[test]
fn can_instantiate_error() {
    assert_eq!(
        ParseError {
            error_kind: ErrorKind::OpenQuote,
            line: String::from("\"This is an example string."),
            column: 27,
        },
        ParseError::new(
            ErrorKind::OpenQuote,
            String::from("\"This is an example string."),
            27
        )
    )
}
