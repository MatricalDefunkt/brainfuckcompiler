#![allow(unused_imports)]

mod lexer;
mod nodes;
mod parser;
mod tokenizer;
use nodes::Nodes;
use parser::Parser;
use std::io::stdin;
use tokenizer::tokenizer::Tokens;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let tokens = Tokens::tokenize(lexer::lexer(&mut input));
    let mut parser = Parser::new(tokens);
    let nodes = parser.parse();

    let mut in_loop: bool;
    let mut field = vec![0; 30000];
    let mut pointer = 0;
    let mut i = 0;

    while i < nodes.len() {
        match nodes[i] {
            Nodes::AddNode => {
                field[pointer] += 1;
            }
            Nodes::SubNode => {
                field[pointer] -= 1;
            }
            Nodes::RightNode => {
                pointer += 1;
            }
            Nodes::LeftNode => {
                pointer -= 1;
            }
            Nodes::LBracketNode => {
                if field[pointer] == 0 {
                    in_loop = true;
                    while in_loop {
                        i += 1;
                        match nodes[i] {
                            Nodes::RBracketNode => {
                                in_loop = false;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Nodes::RBracketNode => {
                if field[pointer] != 0 {
                    in_loop = true;
                    while in_loop {
                        i -= 1;
                        match nodes[i] {
                            Nodes::LBracketNode => {
                                in_loop = false;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Nodes::DotNode => {
                print!("{}", field[pointer] as u8 as char);
            }
            Nodes::CommaNode => {
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                field[pointer] = input.chars().next().unwrap() as u8 as i32;
            }
        }
        i += 1;
    }
    println!(
        "{:?}",
        field
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    // println!("{:?}", nodes);
}
