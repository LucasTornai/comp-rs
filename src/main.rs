extern crate pest;
#[macro_use]
extern crate pest_derive;

mod c_parser;
mod ast_parser;

use c_parser::CParser;
use ast_parser::AstParser;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let path = env::args().nth(1).unwrap();
    
    let ast = AstParser::parse(&path);
    let parsed_to_c = CParser::parse(ast);
    println!("{}", parsed_to_c);

    let mut output_file = File::create("out.c")?;
    output_file.write_all(parsed_to_c.as_bytes())?;
    Ok(())
}
