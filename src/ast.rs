type Nodes = Vec<AstNode>;

#[derive(Debug)]
pub enum AstNode {
    Token(String),
    Options(Nodes),
    Sequence(Nodes),
    Rule(String, Box<AstNode>),
    Grammar(Vec<AstNode>),
}
