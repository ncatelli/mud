use errors::*;

#[test]
fn can_instantiate_error() {
    assert_eq!(
        ParseError {
            error_kind: ErrorKind::EOI,
            cause: String::from("\"This is an example string.")
        },
        ParseError::new(ErrorKind::EOI, String::from("\"This is an example string."))
    )
}
