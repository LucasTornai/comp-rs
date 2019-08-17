extern crate pest;
#[macro_use]
extern crate pest_derive;

mod c_parser;

use c_parser::transpiler::LangC;
use std::env;

fn main() {
    let path = env::args().nth(1).unwrap();

    let ast = LangC::parse_to_ast(&path);
    let parsed_to_c = LangC::parse_to_c(ast);
    println!("{}", parsed_to_c);
}
