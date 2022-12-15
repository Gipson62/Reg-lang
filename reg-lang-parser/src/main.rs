#![allow(unused)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "./src/grammar.pest"]
struct MyParser;

fn main() {
    let tralalalala = MyParser::parse(Rule::file, "5+5*2")
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    print!("{:?}", tralalalala);
}