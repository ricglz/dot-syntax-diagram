use dot_graph::{Edge, Graph, Kind, Node};

use crate::ast::{AstNode, Rule};

struct Generator {
    counter: usize,
    graph: Graph,
}

impl Generator {
    fn create_node(&mut self, label: &str) -> Node {
        let name = &format!("_{}", self.counter);
        self.counter += 1;
        Node::new(name).label(label)
    }

    fn generate_node(&mut self, node: &AstNode) -> Node {
        match node {
            AstNode::Token(token) => self.create_node(token),
            AstNode::Id(id) => self.create_node(id).shape(Some("box")),
            kind => unreachable!("{kind:?}"),
        }
    }

    fn add_node_and_edge(&mut self, node: Node, from: &str) {
        let edge = Edge::new(from, &node.name, "");
        self.graph.add_node(node);
        self.graph.add_edge(edge);
    }

    fn parse_node(&mut self, node: &AstNode, from: &str) {
        match node {
            AstNode::Token(_) | AstNode::Id(_) => {
                let node = self.generate_node(node);
                self.add_node_and_edge(node, from)
            }
            AstNode::Sequence(tokens) => {
                let nodes: Vec<_> = tokens.iter().map(|v| self.generate_node(v)).collect();
                let mut from = from;
                nodes.iter().for_each(|v| {
                    self.add_node_and_edge(v.clone(), from);
                    from = &v.name;
                })
            }
            AstNode::Options(tokens) => {
                let nodes: Vec<_> = tokens.iter().map(|v| self.generate_node(v)).collect();
                nodes.iter().for_each(|v| {
                    self.add_node_and_edge(v.clone(), from);
                })
            }
        };
    }

    fn build_rule(&mut self, rule: &Rule) {
        let node = self.create_node(&rule.0).shape(Some("none"));
        let name = &node.name.clone();
        self.graph.add_node(node);
        self.parse_node(&rule.1, name)
    }

    fn build(&mut self, rules: &[Rule]) {
        rules.iter().for_each(|v| self.build_rule(v))
    }

    #[inline]
    fn to_dot_string(&self) -> String {
        self.graph.to_dot_string().unwrap()
    }

    #[inline]
    fn print_dot_string(&self) {
        println!("{}", self.to_dot_string())
    }
}

impl Default for Generator {
    #[inline]
    fn default() -> Self {
        Generator {
            counter: 0,
            graph: Graph::new("gramar", Kind::Digraph),
        }
    }
}

pub fn generate(rules: &[Rule]) -> String {
    let mut generator = Generator::default();
    generator.build(rules);
    generator.print_dot_string();
    generator.to_dot_string()
}
