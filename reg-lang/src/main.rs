#![allow(unused)]

use std::process::exit;
use clap::{App, Arg, SubCommand};
use reg_lang_parser::tralala;

fn main() {
    tralala("5+5*2");
}
