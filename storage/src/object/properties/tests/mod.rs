use object::properties;

#[test]
fn validate_property_key_passes_valid_keys() {
    let valid_key = "hello";

    assert!(properties::validate_property_key(&valid_key));
}

#[test]
fn validate_property_key_fails_invalid_keys() {
    let invalid_key = "hello world";

    assert!(!properties::validate_property_key(&invalid_key));
}
