use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "program.pest"]
pub struct Analyzer;

#[derive(Debug)]
pub enum AstNode {
    MainFunction(Vec<Box<AstNode>>),
    VariableDeclaration (Vec<Variable>),
    ConditionalStatement {
        bool_expr: String,
        body: Vec<AstNode>,
        else_stmt: Option<Vec<AstNode>>
    },
    WhileStatement {
        bool_expr: String,
        body: Vec<AstNode>
    }
}

#[derive(Debug)]
pub struct Variable {
    pub var_type: String,
    pub ident: String,
    pub value: Option<ValueType>
}

#[derive(Debug)]
pub enum ValueType {
    Ident(String),
    Str(String),
    Float(f32),
    Expression(Vec<Expr>)
}

#[derive(Debug)]
pub enum Expr {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power {
        base: Box<ValueType>,
        exp: Box<ValueType>
    },
    Term(Box<ValueType>)
}

pub struct AstParser;

impl AstParser {
    pub fn parse(path: &str) -> Vec<AstNode> {
        let mut ast = vec![];

        let unparsed_file = fs::read_to_string(&path).expect("cannot read file");
        let pairs = Analyzer::parse(Rule::program, &unparsed_file).unwrap();

        println!("{:#?}", pairs);

        for pair in pairs {
            let n = AstParser::parse_to_ast_node(pair);
            ast.push(n);
        }
        
        return ast;
    }

    fn parse_numerical_expr(pair: pest::iterators::Pair<Rule>) -> ValueType {
        match pair.as_rule() {
            Rule::flt_value => {
                let parsed_value = pair.as_str().parse::<f32>();

                match parsed_value {
                    Ok(v) => ValueType::Float(v),
                    Err(e) => panic!("Error converting {:?} to float - {:?}", pair.as_str(), e)
                }
            },
            Rule::expr => {
                let mut expr: Vec<Expr> = vec![];

                let mut inner_pair = pair.into_inner();

                loop {
                    match inner_pair.next() {
                        Some(pair) => {
                            let e = match pair.as_rule() {
                                Rule::add => Expr::Add,
                                Rule::subtract => Expr::Subtract,
                                Rule::multiply => Expr::Multiply,
                                Rule::divide => Expr::Divide,
                                Rule::power => {
                                    let base = expr.pop().unwrap();
                                    let exp = AstParser::parse_numerical_expr(inner_pair.next().unwrap());

                                    match base {
                                        Expr::Term(term) => {
                                            Expr::Power {
                                                base: term,
                                                exp: Box::new(exp)
                                            }
                                        },
                                        _ => panic!("Unexpected value as base value for exponencial function")
                                    }
                                },
                                _ => {
                                    let value_type = AstParser::parse_numerical_expr(pair);
                                    Expr::Term(Box::new(value_type))
                                }
                            };

                            expr.push(e);
                        },
                        None => break
                    }
                }

                ValueType::Expression(expr)
            },
            _ => panic!("Error in parse_numerical_expr")
        }
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
                            Some(ValueType::Str(String::from(v.as_str())))
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
            Rule::flt_decl => {
                let mut nodes = vec![];

                for inner_pair in pair.into_inner() {
                    let mut pair = inner_pair.into_inner();
                    let ident: pest::iterators::Pair<Rule> = pair.next().unwrap();
                    let ident = String::from(ident.as_str());

                    let value = match pair.next() {
                        Some(p) => Some(AstParser::parse_numerical_expr(p)),
                        None => None
                    };

                    let node = Variable {
                        var_type: String::from("float"),
                        ident,
                        value
                    };

                    nodes.push(node);
                }

                AstNode::VariableDeclaration(nodes)
            },
            Rule::condition_statement => {
                let mut inner_pair = pair.into_inner();
                let if_stmt = inner_pair.next().unwrap();

                let mut if_stmt_inner = if_stmt.into_inner();
                let bool_expr = if_stmt_inner.next().unwrap().as_str();
                let bool_expr = String::from(bool_expr);

                let mut if_body = vec![];

                for pair in if_stmt_inner {
                    let node = AstParser::parse_to_ast_node(pair);
                    if_body.push(node);
                }

                let else_stmt = match inner_pair.next() {
                    Some(p) => {
                        let mut body = vec![];
                        
                        for pair in p.into_inner() {
                            let node = AstParser::parse_to_ast_node(pair);
                            body.push(node);
                        }

                        Some(body)
                    },
                    None => None
                };
                
                AstNode::ConditionalStatement {
                    bool_expr,
                    body: if_body,
                    else_stmt
                }
            },
            Rule::while_statement => {
                let mut while_stmt = pair.into_inner();

                let bool_expr = while_stmt.next().unwrap().as_str();
                let bool_expr = String::from(bool_expr);

                let mut body = vec![];

                for pair in while_stmt {
                    let node = AstParser::parse_to_ast_node(pair);
                    body.push(node);
                }

                AstNode::WhileStatement {
                    bool_expr,
                    body
                }
            }
            Rule::main_func => {
                let mut ast = vec![];

                for pair in pair.into_inner() {
                    let n = AstParser::parse_to_ast_node(pair);
                    ast.push(Box::new(n));
                }

                AstNode::MainFunction(ast)
            },
            _ => panic!("ERROR {:?}", pair)
        }
    }
}