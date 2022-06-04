type Nodes = Vec<AstNode>;

pub enum AstNode {
    Token(String),
    Id(String),
    Options(Nodes),
    Sequence(Nodes),
}

impl core::fmt::Debug for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(val) => write!(f, "Token({val})"),
            Self::Id(val) => write!(f, "Id({val})"),
            Self::Options(nodes) => write!(f, "Options({nodes:#?})"),
            Self::Sequence(nodes) => write!(f, "Sequence({nodes:#?})"),
        }
    }
}

pub struct Rule(pub String, pub AstNode);

impl core::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rule({}, {:?})", self.0, self.1)
    }
}
