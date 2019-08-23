use crate::ast_parser::{ AstNode, ValueType, Expr, IOFunc };

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
                    #include <math.h>
                    int main() {{
                        {b}
                        return 0;
                    }}", b=body)
            },
            AstNode::VariableDeclaration(variables) => {
                let mut declarations = String::new();

                for var in variables {
                    match var.value {
                        Some(value) => declarations.push_str(&format!("{} {}={};\n", var.var_type, var.ident, CParser::parse_variable_value(value))),
                        None => declarations.push_str(&format!("{} {};\n", var.var_type, var.ident))
                    };
                };

                return declarations;
            },
            AstNode::VariableAssignment { ident, value } => {
                format!("{} = {};\n", ident, CParser::parse_variable_value(value.unwrap()))
            },
            AstNode::ConditionalStatement { bool_expr, body, else_stmt } => {
                let mut if_body = String::new();

                for ast_node in body {
                    let expr = CParser::parse_to_c_expr(ast_node);
                    if_body.push_str(&expr);
                }

                match else_stmt {
                    Some(nodes) => {
                        let mut else_body = String::new();

                        for ast_node in nodes {
                            let expr = CParser::parse_to_c_expr(ast_node);
                            else_body.push_str(&expr);
                        }

                        format!("if ({}) {{ {} }} else {{ {} }}\n", bool_expr, if_body, else_body)
                    },
                    None => format!("if ({}) {{ {} }}\n", bool_expr, if_body)
                }
            },
            AstNode::WhileStatement { bool_expr, body } => {
                let mut while_body = String::new();

                for ast_node in body {
                    let expr = CParser::parse_to_c_expr(ast_node);
                    while_body.push_str(&expr);
                }

                format!("
                while ({})  
                {{
                    {}
                }}\n", bool_expr, while_body)
            },
            AstNode::IOStatement { io_stmt, param } => {
                match io_stmt {
                    IOFunc::ReadStr => format!("scanf(\"%s\", &{});\n", param),
                    IOFunc::ReadNum => format!("scanf(\"%f\", &{});\n", param),
                    IOFunc::PrintStr => format!("printf(\"%s\", {});\n", param),
                    IOFunc::PrintNum => format!("printf(\"%f\", {});\n", param)
                }
            },
            AstNode::PrintStatement { printable } => {
                format!("printf({});\n", printable)
            }
        }
    }

    fn parse_variable_value(value: ValueType) -> String {
        match value {
            ValueType::Str(v) => v,
            ValueType::Float(v) => format!("{}", v),
            ValueType::Ident(v) => format!("{}", v),
            ValueType::Expression(expr) => {
                let mut parsed_expr = String::new();

                for e in expr {
                    let parsed = match e {
                        Expr::Add => String::from("+"),
                        Expr::Divide => String::from("/"),
                        Expr::Multiply => String::from("*"),
                        Expr::Subtract => String::from("-"),
                        Expr::Power { base, exp } => {
                            let base = CParser::parse_variable_value(*base);
                            let exp = CParser::parse_variable_value(*exp);
                            format!("pow({}, {})", base, exp)
                        }
                        Expr::Term(expr) => {
                            let parsed_expr = CParser::parse_variable_value(*expr);
                            format!("{}", parsed_expr)
                        }
                    };

                    parsed_expr.push_str(&parsed);
                }

                return format!("({})", parsed_expr);
            }
        }
    }
}