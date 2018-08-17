pub enum Token {
    Call,
    Args,
    Operator(char),
    Operand(ComplexType),
}

enum ComplexType {
    Array(Vec<Primitive>),
    Primitive(Primitive),
}

enum Primitive {
    Float(f64),
    Int(i64),
    Str(&str),
}
