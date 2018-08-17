mod super::lexer;

#[derive(Debug)]
struct Node {
    entry: lexer::Token,
    left: Node,
    right: Node,
}

impl Node {
    pub fn new() -> Node {
        Node {
            entry: lexer::Token,
        }
    }
}
