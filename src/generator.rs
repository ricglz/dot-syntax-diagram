use dot_graph::{Edge, Graph, Kind, Node};

use crate::ast::AstNode;

fn create_edge(a: &Node, b: &Node) -> Edge {
    Edge::new(&a.name, &b.name, "")
}

fn generate_node(node: &AstNode, prefix: &str) -> Node {
    match node {
        AstNode::Token(token) => Node::new(&format!("{prefix}_{token}")).label(token),
        AstNode::Id(id) => Node::new(&format!("{prefix}_{id}"))
            .shape(Some("box"))
            .label(id),
        kind => unreachable!("{kind:?}"),
    }
}

fn build_graph(node: &AstNode, graph: &mut Graph, prefix: &str) {
    match node {
        AstNode::Sequence(exprs) => {
            let nodes: Vec<_> = exprs.iter().map(|v| generate_node(v, prefix)).collect();
            nodes.clone().into_iter().for_each(|v| graph.add_node(v));
            nodes.windows(2).for_each(|window| {
                debug_assert!(window.len() == 2);
                let first = &window[0];
                let second = &window[1];
                graph.add_edge(create_edge(first, second))
            });
            let first = nodes.get(0).unwrap();
            graph.add_edge(Edge::new(prefix, &first.name, ""))
        }
        AstNode::Options(_) => todo!(),
        AstNode::Token(_) | AstNode::Id(_) => {
            let graph_node = generate_node(node, prefix);
            let edge = Edge::new(prefix, &graph_node.name, "");
            graph.add_node(graph_node);
            graph.add_edge(edge)
        }
        kind => unreachable!("{kind:?}"),
    }
}

pub fn build_rule(node: &AstNode, graph: &mut Graph) {
    match node {
        AstNode::Rule(rule_name, value) => {
            graph.add_node(Node::new(rule_name).shape(Some("none")));
            build_graph(value, graph, rule_name)
        }
        kind => unreachable!("{kind:?}"),
    }
}

pub fn generate(node: &AstNode) {
    match node {
        AstNode::Grammar(rules) => {
            let mut graph = Graph::new("grammar", Kind::Digraph);
            rules.iter().for_each(|v| build_rule(v, &mut graph));
            println!("{}", graph.to_dot_string().unwrap());
        }
        kind => unreachable!("{kind:?}"),
    }
}
