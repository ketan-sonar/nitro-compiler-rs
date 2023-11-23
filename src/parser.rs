use std::process::exit;

use crate::tokenizer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum NodeExpr {
    IntLiteral(Token),
    Ident(Token),
}

#[derive(Debug, Clone)]
pub struct NodeStmtExit {
    pub expr: NodeExpr,
}

#[derive(Debug, Clone)]
pub struct NodeStmtLet {
    pub ident: Token,
    pub expr: NodeExpr,
}

#[derive(Debug, Clone)]
pub enum NodeStmt {
    Exit(NodeStmtExit),
    Let(NodeStmtLet),
}

#[derive(Debug)]
pub struct NodeProg {
    pub stmts: Vec<NodeStmt>,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse_expr(&mut self) -> Option<NodeExpr> {
        if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::IntLiteral {
            return Some(NodeExpr::IntLiteral(self.consume()));
        }
        if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::Ident {
            return Some(NodeExpr::Ident(self.consume()));
        }
        None
    }

    pub fn parse_stmt(&mut self) -> Option<NodeStmt> {
        let token_type = self.peek().unwrap().type_;
        if token_type == TokenType::Exit {
            self.consume();
            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::LParen {
                self.consume();
            } else {
                eprintln!("ERROR: `(` expected");
                exit(1);
            }

            let expr = self.parse_expr();
            if expr.is_none() {
                eprintln!("ERROR: could not parse expression");
                exit(1);
            }

            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::RParen {
                self.consume();
            } else {
                eprintln!("ERROR: `)` expected");
                exit(1);
            }

            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::SemiColon {
                self.consume();
            } else {
                eprintln!("ERROR: `;` expected");
                exit(1);
            }

            return Some(NodeStmt::Exit(NodeStmtExit {
                expr: expr.unwrap(),
            }));
        } else if token_type == TokenType::Let {
            self.consume();
            let ident: Token;
            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::Ident {
                ident = self.consume();
            } else {
                eprintln!("ERROR: `identifier` expected");
                exit(1);
            }

            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::Eq {
                self.consume();
            } else {
                eprintln!("ERROR: `=` expected");
                exit(1);
            }

            let expr = self.parse_expr();
            if expr.is_none() {
                eprintln!("ERROR: `expression` expected");
                exit(1);
            }

            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::SemiColon {
                self.consume();
            } else {
                eprintln!("ERROR: `;` expected");
                exit(1);
            }

            return Some(NodeStmt::Let(NodeStmtLet { ident, expr: expr.unwrap() }));
        }

        None
    }

    pub fn parse(&mut self) -> NodeProg {
        let mut node_prog = NodeProg { stmts: vec![] };

        while self.peek().is_some() {
            let stmt = self.parse_stmt();
            if stmt.is_none() {
                eprintln!("ERROR: could not parse statement\n");
                exit(1);
            }
            node_prog.stmts.push(stmt.unwrap());
        }

        node_prog
    }

    fn consume(&mut self) -> Token {
        self.index += 1;
        self.tokens[self.index - 1].clone()
    }

    fn peek(&self) -> Option<Token> {
        if self.index >= self.tokens.len() {
            return None;
        }

        Some(self.tokens[self.index].clone())
    }
}
