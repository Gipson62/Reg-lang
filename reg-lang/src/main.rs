#![allow(unused)]

use std::process::exit;
use clap::{App, Arg, SubCommand};
use reg_lang_parser::*;

fn main() {
    parse("4+5*5");
}   