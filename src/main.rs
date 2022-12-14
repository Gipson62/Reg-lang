#![allow(unused)]
use std::alloc;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar/grammar.pest"]
pub(crate) struct RegParser;
fn main() {
    println!("{}", "-52".parse::<i32>().unwrap())
}