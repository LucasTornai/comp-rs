extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;
use std::env;

#[derive(Parser)]
#[grammar = "program.pest"]
pub struct Analyzer;

fn main() {
    let path = env::args().nth(1).unwrap();
    let unparsed_file = fs::read_to_string(&path).expect("cannot read file");

    let successful_parse = Analyzer::parse(Rule::program, &unparsed_file);
    println!("{:?}", successful_parse.unwrap());
}
