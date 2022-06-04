use dot_graph::{Edge, Graph, Kind, Node};

use crate::ast::AstNode;

fn generate_node(node: &AstNode) -> Node {
    match node {
        AstNode::Token(token) => Node::new(token),
        AstNode::Id(id) => Node::new(id).shape(Some("box")),
        kind => unreachable!("{kind:?}"),
    }
}

fn build_graph(node: &AstNode, graph: &mut Graph) {
    match node {
        AstNode::Sequence(exprs) => {
            let nodes: Vec<_> = exprs.iter().map(generate_node).collect();
            nodes.clone().into_iter().for_each(|v| graph.add_node(v));
            nodes.windows(2).for_each(|window| {
                debug_assert!(window.len() == 2);
                let first = &window[0];
                let second = &window[1];
                graph.add_edge(Edge::new(&first.name, &second.name, ""))
            })
        }
        AstNode::Options(_) => todo!(),
        AstNode::Token(_) | AstNode::Id(_) => {
            let graph_node = generate_node(node);
            graph.add_node(graph_node);
        }
        kind => unreachable!("{kind:?}"),
    }
}

pub fn generate(node: &AstNode) {
    match node {
        AstNode::Grammar(rules) => rules.iter().for_each(generate),
        AstNode::Rule(name, value) => {
            let mut graph = Graph::new(name, Kind::Digraph);
            build_graph(value, &mut graph);
            println!("{}", graph.to_dot_string().unwrap());
        }
        kind => unreachable!("{kind:?}"),
    }
}
