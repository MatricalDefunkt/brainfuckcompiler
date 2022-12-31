#![allow(unused_imports)]

mod lexer;
mod nodes;
mod parser;
mod tokenizer;
use nodes::Nodes;
use parser::Parser;
use std::io::{stdin, Write};
use tokenizer::tokenizer::Tokens;

fn main() {
    println!("Welcome to the BrainFuck interpreter!\nPlease choose: \n1) Path to BF file \n2) Inputting a BF string directly");
    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().parse::<i32>().unwrap();
    match choice {
        1 => {
            println!("Please enter the path to the file:");
            let mut path = String::new();
            stdin().read_line(&mut path).expect("Failed to read line");
            let path = path.trim();

            let mut file = std::fs::File::open(path).expect("File not found");
            let mut contents = String::new();
            std::io::Read::read_to_string(&mut file, &mut contents)
                .expect("Something went wrong reading the file");

            let tokens = Tokens::tokenize(lexer::lexer(&mut contents));
            let mut parser = Parser::new(tokens);
            let nodes = parser.parse();

            run(nodes);
        }
        2 => {
            println!("Please enter the string:");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let tokens = Tokens::tokenize(lexer::lexer(&mut input));
            let mut parser = Parser::new(tokens);
            let nodes = parser.parse();

            run(nodes);
        }
        _ => {
            println!("Invalid choice");
        }
    }
}

fn run(nodes: Vec<Nodes>) {
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
            Nodes::HashNode => {
                print!("{} ", field[pointer]);
            }
        }
        i += 1;
    }
}
