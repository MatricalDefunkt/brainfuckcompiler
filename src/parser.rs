use crate::nodes::Nodes;
use crate::tokenizer::tokenizer::Tokens;

pub struct Parser {
    pub tokens: Vec<Tokens>,
    pub nodes: Vec<Nodes>,
}

impl Parser {
    pub fn new(tokens: Vec<Tokens>) -> Parser {
        Parser {
            tokens,
            nodes: vec![],
        }
    }

    pub fn parse(&mut self) -> Vec<Nodes> {
        for token in self.tokens.iter() {
            match token {
                Tokens::Plus => self.nodes.push(Nodes::AddNode),
                Tokens::Minus => self.nodes.push(Nodes::SubNode),
                Tokens::Right => self.nodes.push(Nodes::RightNode),
                Tokens::Left => self.nodes.push(Nodes::LeftNode),
                Tokens::LBracket => self.nodes.push(Nodes::LBracketNode),
                Tokens::RBracket => self.nodes.push(Nodes::RBracketNode),
                Tokens::Dot => self.nodes.push(Nodes::DotNode),
                Tokens::Comma => self.nodes.push(Nodes::CommaNode),
                Tokens::Hash => self.nodes.push(Nodes::HashNode),
            }
        }
        return self.nodes.clone();
    }
}
