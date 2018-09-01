extern crate parser;

#[test]
fn parse_returns_ok_on_valid_statment() {
    let full_sentence = "take pencil from table";
    match parser::parse(full_sentence.to_string()) {
        Ok(_) => (),
        Err(_) => panic!("Parse(\"{}\") returns an Err.", full_sentence),
    }
}
