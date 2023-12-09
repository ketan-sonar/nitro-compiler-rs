use std::{collections::HashMap, process::exit};

use crate::parser::{NodeProg, NodeStmt, NodeExpr};

const REG_SIZE: usize = 16;

#[derive(Debug)]
pub struct Var {
    stack_loc: usize,
}

pub struct Generator {
    prog: NodeProg,
    output: String,
    vars: HashMap<String, Var>,
    stack_size: usize,
}

impl Generator {
    pub fn new(prog: NodeProg) -> Self {
        Self {
            prog,
            output: String::new(),
            vars: HashMap::new(),
            stack_size: 0
        }
    }

    pub fn generate_expr(&mut self, expr: Box<NodeExpr>) {
        match *expr {
            NodeExpr::IntLiteral(int_literal) => {
                let value = int_literal.value.unwrap();
                let inst = format!("    mov X0, #{}\n", value);
                self.output.push_str(&inst);
                self.push("X0");
            }
            NodeExpr::Ident(ident_token) => {
                let ident = ident_token.value.unwrap();
                if !self.vars.contains_key(&ident) {
                    eprintln!("Identifier `{}` not found", ident);
                    exit(1);
                }

                let offset = self.stack_size - self.vars[&ident].stack_loc - 1;
                let reg = format!("[sp, #0x{:X}]", offset * REG_SIZE);
                let inst = format!("    ldr X0, {}\n", reg);
                self.output.push_str(&inst);
                self.push("X0");
            }
            NodeExpr::BinExpr(_bin_expr) => {
                unimplemented!();
            }
        }
    }

    pub fn generate_stmt(&mut self, stmt: NodeStmt) {
        match stmt {
            NodeStmt::Exit(exit_stmt) => {
                self.generate_expr(exit_stmt.expr);
                self.pop("X0");
                self.output.push_str("    mov X16, #1\n");
                self.output.push_str("    svc #0x80\n");
            }
            NodeStmt::Let(let_stmt) => {
                let ident = let_stmt.ident.value.unwrap();
                if self.vars.contains_key(&ident) {
                    eprintln!("ERROR: identifier `{}` already exists", ident);
                    exit(1);
                }

                self.generate_expr(let_stmt.expr.clone());

                self.vars.insert(ident, Var { stack_loc: self.stack_size });
                self.stack_size += 1;
            }
        }
    }

    pub fn generate(&mut self) -> String {
        self.output.push_str(".global _start\n");
        self.output.push_str(".align 2\n\n");
        self.output.push_str("_start:\n");

        for i in 0..self.prog.stmts.len() {
            let stmt = self.prog.stmts[i].clone();
            self.generate_stmt(stmt);
        }

        self.output.push_str("    mov X0, #0\n");
        self.output.push_str("    mov X16, #1\n");
        self.output.push_str("    svc #0x80\n");

        self.output.clone()
    }

    fn push(&mut self, reg: &str) {
        let inst = format!("    str {}, [sp, #-0x{:X}]!\n", reg, REG_SIZE);
        self.output.push_str(&inst);
    }

    fn pop(&mut self, reg: &str) {
        let inst = format!("    ldr {}, [sp]\n", reg);
        self.output.push_str(&inst);
    }
}
