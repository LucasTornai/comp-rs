use pest::Parser;
use std::fs;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "program.pest"]
pub struct Analyzer;

#[derive(Debug)]
pub enum AstNode {
    MainFunction(Vec<Box<AstNode>>),
    VariableDeclaration (Vec<Variable>),
    VariableAssignment {
        ident: String,
        value: Option<ValueType>
    },
    ConditionalStatement {
        bool_expr: String,
        body: Vec<AstNode>,
        else_stmt: Option<Vec<AstNode>>
    },
    WhileStatement {
        bool_expr: String,
        body: Vec<AstNode>
    },
    IOStatement {
        io_stmt: IOFunc,
        param: String
    },
    PrintStatement {
        printable: String
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

#[derive(Debug)]
pub enum IOFunc {
    ReadStr,
    ReadNum,
    PrintStr,
    PrintNum
}

pub struct AstParser;

impl AstParser {
    pub fn new_var(variables: &mut HashMap<String, String>, ident: &String, var_type: &String) {
        if variables.contains_key(ident) {
            panic!("Variable {} already declared", ident);
        } else {
            variables.insert(ident.to_string(), var_type.to_string());
        }
    }

    pub fn assign_var(variables: &mut HashMap<String, String>, ident: &String, var_type: &String) {
        if variables.contains_key(ident) {
            let old_type = variables.get(ident).unwrap();
            if old_type != var_type {
                panic!("Trying to assign value of type {} to variable {} of type {}", var_type, ident, old_type);
            } else {
                return
            }
        } else {
            panic!("Variable {} was not initialized", ident);
        }
    }

    pub fn parse(path: &str) -> Vec<AstNode> {
        let mut ast = vec![];
        let mut variables = HashMap::new();

        let unparsed_file = fs::read_to_string(&path).expect("cannot read file");
        let parsed_file = Analyzer::parse(Rule::program, &unparsed_file);

        match parsed_file {
            Ok(_) => print!("Ok file!"),
            Err(_) => panic!("Syntax error!")
        }

        let pairs = parsed_file.unwrap();

        println!("{:#?}", pairs);

        for pair in pairs {
            let n = AstParser::parse_to_ast_node(&mut variables, pair);
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
            Rule::t_id => {
                let parsed_value = pair.as_str().parse::<String>();

                match parsed_value {
                    Ok(v) => ValueType::Ident(v),
                    Err(e) => panic!("Error converting {:?} to ident - {:?}", pair.as_str(), e)
                }
            }
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

    fn parse_to_ast_node(variables: &mut HashMap<String, String>, pair: pest::iterators::Pair<Rule>) -> AstNode {
        match pair.as_rule() {
            Rule::str_decl => {
                let mut nodes = vec![];

                let mut inner_pair = pair.into_inner();
                let ident = inner_pair.next().unwrap();
                let mut ident = String::from(ident.as_str());

                let value = match inner_pair.next() {
                    Some(v) => {
                         ident = format!("{}", ident);
                         Some(ValueType::Str(String::from(v.as_str())))
                    },
                    None => {
                        ident = format!("{}", ident);
                        None
                    }
                };

                let var_type = String::from("char*");

                let node = Variable {
                    var_type: var_type.clone(),
                    ident: ident.clone(),
                    value
                };
                    
                AstParser::new_var(variables, &ident, &var_type);
                nodes.push(node);

                return AstNode::VariableDeclaration(nodes);
            },
            Rule::flt_decl => {
                let mut nodes = vec![];

                let mut inner_pair = pair.into_inner();
                let ident = inner_pair.next().unwrap();
                let ident = String::from(ident.as_str());

                let value = match inner_pair.next() {
                    Some(p) => Some(AstParser::parse_numerical_expr(p)),
                    None => None
                };

                let var_type = String::from("float");

                let node = Variable {
                    var_type: var_type.clone(),
                    ident: ident.clone(),
                    value
                };

                AstParser::new_var(variables, &ident, &var_type);
                nodes.push(node);

                AstNode::VariableDeclaration(nodes)
            },
            Rule::str_assign => {
                let mut inner_pair = pair.into_inner();
                
                let ident = inner_pair.next().unwrap();
                let ident = String::from(ident.as_str());

                AstParser::assign_var(variables, &ident, &String::from("char*"));

                let value = match inner_pair.next() {
                    Some(v) => {
                         Some(ValueType::Str(String::from(v.as_str())))
                    },
                    None => {
                        panic!("No value in string variable {} assignment!", ident)
                    }
                };

                AstNode::VariableAssignment {
                    ident,
                    value
                }
            },
            Rule::flt_assign => {
                let mut inner_pair = pair.into_inner();

                let ident = inner_pair.next().unwrap();
                let ident = String::from(ident.as_str());

                AstParser::assign_var(variables, &ident, &String::from("float"));

                let value = match inner_pair.next() {
                    Some(p) => Some(AstParser::parse_numerical_expr(p)),
                    None => panic!("No value in float variable {} assignment!", ident)
                };

                AstNode::VariableAssignment {
                    ident,
                    value
                }
            },
            Rule::condition_statement => {
                let mut inner_pair = pair.into_inner();
                let if_stmt = inner_pair.next().unwrap();

                let mut if_stmt_inner = if_stmt.into_inner();
                let bool_expr = if_stmt_inner.next().unwrap().as_str();
                let bool_expr = String::from(bool_expr);

                let mut if_body = vec![];

                for pair in if_stmt_inner {
                    let node = AstParser::parse_to_ast_node(variables, pair);
                    if_body.push(node);
                }

                let else_stmt = match inner_pair.next() {
                    Some(p) => {
                        let mut body = vec![];
                        
                        for pair in p.into_inner() {
                            let node = AstParser::parse_to_ast_node(variables, pair);
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
                    let node = AstParser::parse_to_ast_node(variables, pair);
                    body.push(node);
                }

                AstNode::WhileStatement {
                    bool_expr,
                    body
                }
            },
            Rule::io_statement => {
                let io_stmt: IOFunc;
                let mut inner_pair = pair.into_inner();
                let inner_pair_next = inner_pair.clone().next();

                match inner_pair_next {
                    Some(pair) => {
                        let e = match pair.as_rule() {
                            Rule::read_string_stmt => IOFunc::ReadStr,
                            Rule::read_num_stmt => IOFunc::ReadNum,
                            Rule::print_string_stmt => IOFunc::PrintStr,
                            Rule::print_num_stmt => IOFunc::PrintNum,
                            _ => panic!("Unexpected IO identifier")
                        };
                        io_stmt = e;
                    },
                    None => panic!("No IO identifier in IO statement")
                }
    
                let param = inner_pair.next().unwrap().into_inner().next().unwrap().as_str();
                let param = String::from(param);
                
                AstNode::IOStatement {
                    io_stmt,
                    param
                }
            },
            Rule::print_stmt => {
                let mut inner_pair = pair.into_inner();

                let printable = inner_pair.next().unwrap().as_str();
                let printable = String::from(printable);

                AstNode::PrintStatement {
                    printable
                }
            },
            Rule::main_func => {
                let mut ast = vec![];

                for pair in pair.into_inner() {
                    let n = AstParser::parse_to_ast_node(variables, pair);
                    ast.push(Box::new(n));
                }

                AstNode::MainFunction(ast)
            },
            _ => panic!("ERROR {:?}", pair)
        }
    }
}