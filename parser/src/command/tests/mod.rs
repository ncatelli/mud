use command::Command;
use lexer::Primitive;

#[test]
fn can_instantiate_command_with_verb() {
    let tokens = vec![Primitive::Symbol(String::from("go"))];

    match Command::new(tokens) {
        Ok(c) => {
            assert_eq!(Some(Primitive::Symbol(String::from("go"))), c.verb());
        }
        Err(e) => panic!(e),
    }
}

#[test]
fn can_instantiate_command_with_verb_and_do() {
    let tokens = vec![
        Primitive::Symbol(String::from("go")),
        Primitive::Symbol(String::from("east")),
    ];
    let com = Command::new(tokens);

    match com {
        Ok(c) => {
            assert_eq!(Some(Primitive::Symbol(String::from("go"))), c.verb());

            assert_eq!(
                Some(Primitive::Symbol(String::from("east"))),
                c.direct_object()
            );
        }
        Err(e) => panic!(e),
    }
}

#[test]
fn can_instantiate_command_with_verb_and_do_prep_io() {
    let tokens = vec![
        Primitive::Symbol(String::from("go")),
        Primitive::Symbol(String::from("east")),
        Primitive::Symbol(String::from("from")),
        Primitive::Symbol(String::from("here")),
    ];
    let com = Command::new(tokens);

    match com {
        Ok(c) => {
            assert_eq!(Some(Primitive::Symbol(String::from("go"))), c.verb());

            assert_eq!(
                Some(Primitive::Symbol(String::from("east"))),
                c.direct_object()
            );

            assert_eq!(
                Some(Primitive::Symbol(String::from("from"))),
                c.preposition()
            );

            assert_eq!(
                Some(Primitive::Symbol(String::from("here"))),
                c.indirect_object()
            );
        }
        Err(e) => panic!(e),
    }
}

#[test]
fn instantiating_a_command_with_three_tokens_should_fail() {
    let tokens = vec![
        Primitive::Symbol(String::from("go")),
        Primitive::Symbol(String::from("east")),
        Primitive::Symbol(String::from("from")),
    ];

    match Command::new(tokens) {
        Ok(_) => panic!("Test should return an Err."),
        Err(_) => (),
    }
}

#[test]
fn instantiating_a_command_with_more_than_four_tokens_should_fail() {
    let tokens = vec![
        Primitive::Symbol(String::from("go")),
        Primitive::Symbol(String::from("east")),
        Primitive::Symbol(String::from("from")),
        Primitive::Symbol(String::from("here")),
        Primitive::Symbol(String::from("again")),
    ];

    match Command::new(tokens) {
        Ok(_) => panic!("Test should return an Err."),
        Err(_) => (),
    }
}
