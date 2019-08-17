pub mod transpiler {
    use pest::Parser;
    use std::fs;

    #[derive(Parser)]
    #[grammar = "program.pest"]
    pub struct Analyzer;

    #[derive(Debug)]
    pub enum AstNode {
        MainFunction(Vec<Box<AstNode>>),
        VariableDeclaration (Vec<Variable>)
    }

    #[derive(Debug)]
    pub struct Variable {
        var_type: String,
        ident: String,
        value: Option<String>
    }
    
    pub struct LangC;

    impl LangC {
        pub fn parse_to_c(nodes: Vec<AstNode>) -> String {
            let mut source_code = String::new();

            for node in nodes {
                let expr = &LangC::parse_to_c_expr(node);
                source_code.push_str(expr);
            }

            return source_code;
        }

        fn parse_to_c_expr(node: AstNode) -> String {
            match node {
                AstNode::MainFunction(nodes) => {
                    let mut body = String::new();

                    for node in nodes {
                        body.push_str(&LangC::parse_to_c_expr(*node));
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

        pub fn parse_to_ast(path: &str) -> Vec<AstNode> {
            let mut ast = vec![];

            let unparsed_file = fs::read_to_string(&path).expect("cannot read file");
            let pairs = Analyzer::parse(Rule::program, &unparsed_file).unwrap();
            
            println!("{:?}", pairs);

            println!("\n\n");

            for pair in pairs {
                let n = LangC::parse_to_ast_node(pair);
                ast.push(n);
            }
            
            return ast;
        }

        fn parse_to_ast_node(pair: pest::iterators::Pair<Rule>) -> AstNode {
            match pair.as_rule() {
                Rule::str_decl => {
                    let mut nodes = vec![];

                    for inner_pair in pair.into_inner() {
                        let mut pair = inner_pair.into_inner();
                        let ident: pest::iterators::Pair<Rule> = pair.next().unwrap();
                        let mut ident = String::from(ident.as_str());

                        let value = match pair.next() {
                            Some(v) => {
                                ident = format!("{}[]", ident);
                                Some(String::from(v.as_str()))
                            },
                            None => {
                                ident = format!("{}[0]", ident);
                                None
                            }
                        };

                        let node = Variable {
                            var_type: String::from("char"),
                            ident,
                            value
                        };

                        nodes.push(node);
                    }

                    return AstNode::VariableDeclaration(nodes);
                },
                Rule::main_func => {
                    let mut ast = vec![];

                    for pair in pair.into_inner() {
                        let n = LangC::parse_to_ast_node(pair);
                        ast.push(Box::new(n));
                    }

                    AstNode::MainFunction(ast)
                },
                _ => panic!("ERROR {:?}", pair)
            }
        }

    }
}