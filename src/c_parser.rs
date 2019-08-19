use crate::ast_parser::AstNode;

pub struct CParser;

impl CParser {
    pub fn parse(nodes: Vec<AstNode>) -> String {
        let mut source_code = String::new();

        for node in nodes {
            let expr = &CParser::parse_to_c_expr(node);
            source_code.push_str(expr);
        }

        return source_code;
    }

    fn parse_to_c_expr(node: AstNode) -> String {
        match node {
            AstNode::MainFunction(nodes) => {
                let mut body = String::new();

                for node in nodes {
                    body.push_str(&CParser::parse_to_c_expr(*node));
                }

                format!("
                    #include <stdio.h>
                    int main() {{
                        {b}
                        return 0;
                    }}", b=body)
            },
            AstNode::VariableDeclaration(variables) => {
                let mut declarations = String::new();

                for var in variables {
                    match &var.value {
                        Some(value) => declarations.push_str(&format!("{} {}={};\n", var.var_type, var.ident, value)),
                        None => declarations.push_str(&format!("{} {};\n", var.var_type, var.ident))
                    };
                };

                return declarations;
            }
        }
    }
}