use pest_consume::match_nodes;
use pest_consume::Parser;

use crate::ast::AstNode;

#[derive(Parser)]
#[grammar = "pest_grammar.pest"] // relative to src
struct PestParser;

use pest_consume::Error;
type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, bool>;
type Rules = Vec<crate::ast::Rule>;

// This is the other half of the parser, using pest_consume.
#[pest_consume::parser]
impl PestParser {
    fn EOI(input: Node) -> Result<()> {
        Ok(())
    }

    fn identifier(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn inner_str(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn terminal(input: Node) -> Result<AstNode> {
        Ok(match_nodes!(input.into_children();
            [inner_str(term)] => AstNode::Token(term),
            [identifier(term)] => AstNode::Id(term),
        ))
    }

    fn term(input: Node) -> Result<AstNode> {
        Ok(match_nodes!(input.into_children();
            [terminal(term)] => term,
            [expression(term)] => term,
        ))
    }

    fn seq(input: Node) -> Result<AstNode> {
        Ok(match_nodes!(input.into_children();
            [term(term)] => term,
            [term(terms)..] => AstNode::Sequence(terms.collect()),
        ))
    }

    fn expression(input: Node) -> Result<AstNode> {
        Ok(match_nodes!(input.into_children();
            [seq(term)] => term,
            [seq(terms)..] => AstNode::Options(terms.collect()),
        ))
    }

    fn grammar_rule(input: Node) -> Result<crate::ast::Rule> {
        Ok(match_nodes!(input.into_children();
            [identifier(id), expression(expr)] => crate::ast::Rule(id, expr),
        ))
    }

    fn grammar_rules(input: Node) -> Result<Rules> {
        Ok(match_nodes!(input.into_children();
            [grammar_rule(rules).., _] => rules.collect(),
        ))
    }
}

pub fn parse(source: &str, debug: bool) -> Result<Rules> {
    let inputs = PestParser::parse_with_userdata(Rule::grammar_rules, source, debug)?;
    // There should be a single root node in the parsed tree
    let input = inputs.single()?;
    PestParser::grammar_rules(input)
}
