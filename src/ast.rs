type Nodes = Vec<AstNode>;

pub enum AstNode {
    Token(String),
    Id(String),
    Options(Nodes),
    Sequence(Nodes),
    Rule(String, Box<AstNode>),
    Grammar(Nodes),
}

impl core::fmt::Debug for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(val) => write!(f, "Token({val})"),
            Self::Id(val) => write!(f, "Id({val})"),
            Self::Options(nodes) => write!(f, "Options({nodes:#?})"),
            Self::Sequence(nodes) => write!(f, "Sequence({nodes:#?})"),
            Self::Rule(id, rule) => write!(f, "Rule({id}, {rule:?})"),
            Self::Grammar(nodes) => write!(f, "Grammar({nodes:#?})"),
        }
    }
}
