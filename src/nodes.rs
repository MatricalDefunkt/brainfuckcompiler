use std::fmt::Debug;
use std::{fmt::Error, fmt::Formatter};
#[derive(Clone)]
pub enum Nodes {
    AddNode,
    SubNode,
    RightNode,
    LeftNode,
    LBracketNode,
    RBracketNode,
    DotNode,
    CommaNode,
}

impl Debug for Nodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Nodes::AddNode => write!(f, "AddNode"),
            Nodes::SubNode => write!(f, "SubNode"),
            Nodes::RightNode => write!(f, "RightNode"),
            Nodes::LeftNode => write!(f, "LeftNode"),
            Nodes::LBracketNode => write!(f, "LBracketNode"),
            Nodes::RBracketNode => write!(f, "RBracketNode"),
            Nodes::DotNode => write!(f, "DotNode"),
            Nodes::CommaNode => write!(f, "CommaNode"),
        }
    }
}
