//! Compiler Service
//! 
//! Translates high-level code or intermediate representations down to
//! the execution engine's Instruction set.

use execution_engine::models::{Instruction, Register};

/// A basic representation of an AST Node for our simplified language
#[derive(Debug, Clone)]
pub enum AstNode {
    Assign(String, Box<AstNode>),
    Add(Box<AstNode>, Box<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),
    Mul(Box<AstNode>, Box<AstNode>),
    Literal(u32),
    Variable(String),
}

/// The Compiler environment for managing variable-to-register allocations
pub struct Compiler {
    allocations: std::collections::HashMap<String, Register>,
    next_free_reg: usize,
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            allocations: std::collections::HashMap::new(),
            next_free_reg: 1, // Start allocating from R1
            instructions: Vec::new(),
        }
    }

    fn alloc_reg(&mut self) -> Result<Register, String> {
        let reg = match self.next_free_reg {
            1 => Register::R1, 2 => Register::R2, 3 => Register::R3,
            4 => Register::R4, 5 => Register::R5, 6 => Register::R6,
            7 => Register::R7, _ => return Err("Out of registers".to_string()),
        };
        self.next_free_reg += 1;
        Ok(reg)
    }

    /// Compiles an AST node into instructions, leaving the result in the returned Register
    pub fn compile_node(&mut self, node: &AstNode) -> Result<Register, String> {
        match node {
            AstNode::Literal(val) => {
                let r = self.alloc_reg()?;
                self.instructions.push(Instruction::Li(r, *val));
                Ok(r)
            }
            AstNode::Variable(name) => {
                if let Some(r) = self.allocations.get(name) {
                    Ok(*r)
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            AstNode::Assign(name, expr) => {
                let r = self.compile_node(expr)?;
                self.allocations.insert(name.clone(), r);
                Ok(r) // Provide assignment result as expression
            }
            AstNode::Add(lhs, rhs) => {
                let r_lhs = self.compile_node(lhs)?;
                let r_rhs = self.compile_node(rhs)?;
                let r_res = self.alloc_reg()?;
                self.instructions.push(Instruction::Add(r_res, r_lhs, r_rhs));
                Ok(r_res)
            }
            AstNode::Sub(lhs, rhs) => {
                let r_lhs = self.compile_node(lhs)?;
                let r_rhs = self.compile_node(rhs)?;
                let r_res = self.alloc_reg()?;
                self.instructions.push(Instruction::Sub(r_res, r_lhs, r_rhs));
                Ok(r_res)
            }
            AstNode::Mul(lhs, rhs) => {
                let r_lhs = self.compile_node(lhs)?;
                let r_rhs = self.compile_node(rhs)?;
                let r_res = self.alloc_reg()?;
                self.instructions.push(Instruction::Mul(r_res, r_lhs, r_rhs));
                Ok(r_res)
            }
        }
    }

    pub fn compile_program(mut self, program: Vec<AstNode>) -> Result<Vec<Instruction>, String> {
        for node in program {
            self.compile_node(&node)?;
        }
        
        // Ensure graceful halt
        self.instructions.push(Instruction::Ecall);
        
        Ok(self.instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use execution_engine::models::{Instruction, Register};

    #[test]
    fn test_compile_addition() {
        let mut compiler = Compiler::new();
        // Test AST: x = 5 + 10
        let ast = AstNode::Assign(
            "x".to_string(),
            Box::new(AstNode::Add(
                Box::new(AstNode::Literal(5)),
                Box::new(AstNode::Literal(10)),
            )),
        );

        let instrs = compiler.compile_program(vec![ast]).unwrap();
        
        // Expected: R1 = 5, R2 = 10, R3 = R1 + R2 (and stored in x), Ecall
        assert_eq!(instrs.len(), 4);
        assert_eq!(instrs[0], Instruction::Li(Register::R1, 5));
        assert_eq!(instrs[1], Instruction::Li(Register::R2, 10));
        assert_eq!(instrs[2], Instruction::Add(Register::R3, Register::R1, Register::R2));
        assert_eq!(instrs[3], Instruction::Ecall);
    }
}

