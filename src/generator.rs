use dot_graph::{Edge, Graph, Kind, Node};

use crate::ast::AstNode;

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

    fn build_graph(&mut self, node: &AstNode, from: &str) {
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
            kind => unreachable!("{kind:?}"),
        };
        println!("{}", self.to_dot_string())
    }

    fn build_rule(&mut self, node: &AstNode) {
        match node {
            AstNode::Rule(rule_name, value) => {
                let node = self.create_node(rule_name).shape(Some("none"));
                let name = &node.name.clone();
                self.graph.add_node(node);
                self.build_graph(value, name)
            }
            kind => unreachable!("{kind:?}"),
        }
    }

    fn to_dot_string(&self) -> String {
        self.graph.to_dot_string().unwrap()
    }

    fn build(&mut self, node: &AstNode) {
        match node {
            AstNode::Grammar(rules) => rules.iter().for_each(|v| self.build_rule(v)),
            kind => unreachable!("{kind:?}"),
        }
    }
}

impl Default for Generator {
    fn default() -> Self {
        Generator {
            counter: 0,
            graph: Graph::new("gramar", Kind::Digraph),
        }
    }
}

pub fn generate(node: &AstNode) {
    let mut generator = Generator::default();
    generator.build(node);
    println!("{}", generator.to_dot_string())
}
