use std::process::exit;

use crate::{tokenizer::{Token, TokenType}, arena::ArenaAllocator};

#[derive(Debug, Clone)]
pub enum NodeExpr {
    IntLiteral(Token),
    Ident(Token),
    BinExpr(Box<NodeBinExpr>),
}

#[derive(Debug, Clone)]
pub struct NodeBinExprAdd {
    lhs: Box<NodeExpr>,
    rhs: Box<NodeExpr>,
}

#[derive(Debug, Clone)]
pub struct NodeBinExprMul {
    lhs: Box<NodeExpr>,
    rhs: Box<NodeExpr>,
}

#[derive(Debug, Clone)]
pub enum NodeBinExpr {
    Add(Box<NodeBinExprAdd>),
    Mul(Box<NodeBinExprMul>),
}

#[derive(Debug, Clone)]
pub struct NodeStmtExit {
    pub expr: Box<NodeExpr>,
}

#[derive(Debug, Clone)]
pub struct NodeStmtLet {
    pub ident: Token,
    pub expr: Box<NodeExpr>,
}

#[derive(Debug, Clone)]
pub enum NodeStmt {
    Exit(Box<NodeStmtExit>),
    Let(Box<NodeStmtLet>),
}

#[derive(Debug)]
pub struct NodeProg {
    pub stmts: Vec<NodeStmt>,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    allocator: ArenaAllocator,
}

const FOUR_MB: usize = 4 * 1024 * 1024;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            allocator: ArenaAllocator::new(FOUR_MB)
        }
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
                Self::error_token_expected("(");
            }

            let expr = self.parse_expr();
            if expr.is_none() {
                eprintln!("ERROR: could not parse expression");
                exit(1);
            }

            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::RParen {
                self.consume();
            } else {
                Self::error_token_expected(")");
            }

            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::SemiColon {
                self.consume();
            } else {
                Self::error_token_expected(";");
            }

            let exit_stmt = Box::new(NodeStmtExit {
                expr: Box::new(expr.unwrap()),
            });

            return Some(NodeStmt::Exit(exit_stmt));
        } else if token_type == TokenType::Let {
            self.consume();
            let mut ident: Token = Token { type_: TokenType::Ident, value: None };
            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::Ident {
                ident.value = self.consume().value;
            } else {
                Self::error_token_expected("identifier");
            }

            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::Eq {
                self.consume();
            } else {
                Self::error_token_expected("=");
            }

            let expr = self.parse_expr();
            if expr.is_none() {
                Self::error_token_expected("expression");
            }

            if self.peek().is_some() && self.peek().unwrap().type_ == TokenType::SemiColon {
                self.consume();
            } else {
                Self::error_token_expected(";");
            }

            let let_stmt = Box::new(NodeStmtLet {
                ident,
                expr: Box::new(expr.unwrap()),
            });

            return Some(NodeStmt::Let(let_stmt));
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

    fn error_token_expected(token: &str) {
        eprintln!("ERROR: `{}` expected", token);
        exit(1);
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
