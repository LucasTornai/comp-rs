extern crate pest;
#[macro_use]
extern crate pest_derive;

mod c_parser;
mod ast_parser;

use c_parser::CParser;
use ast_parser::AstParser;
use std::env;

fn main() {
    let path = env::args().nth(1).unwrap();

    let ast = AstParser::parse(&path);
    let parsed_to_c = CParser::parse(ast);
    println!("{}", parsed_to_c);
}
