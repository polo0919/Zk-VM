//! RV32IM RISC-V Execution Models
//! 
//! Defines the core architectural state for a True RISC-V 32-bit emulator. 
//! Captures comprehensive execution traces mapping instruction decoding and memory
//! for rigorous cryptographic proving via AIR constraints.

use std::collections::BTreeMap;

/// 32 General Purpose Registers in the RV32IM architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30, X31,
}

impl Register {
    pub fn from_u32(idx: u32) -> Self {
        match idx {
            0 => Register::X0, 1 => Register::X1, 2 => Register::X2, 3 => Register::X3,
            4 => Register::X4, 5 => Register::X5, 6 => Register::X6, 7 => Register::X7,
            8 => Register::X8, 9 => Register::X9, 10 => Register::X10, 11 => Register::X11,
            12 => Register::X12, 13 => Register::X13, 14 => Register::X14, 15 => Register::X15,
            16 => Register::X16, 17 => Register::X17, 18 => Register::X18, 19 => Register::X19,
            20 => Register::X20, 21 => Register::X21, 22 => Register::X22, 23 => Register::X23,
            24 => Register::X24, 25 => Register::X25, 26 => Register::X26, 27 => Register::X27,
            28 => Register::X28, 29 => Register::X29, 30 => Register::X30, 31 => Register::X31,
            _ => Register::X0, // Hardwired zero by default on out-of-bounds
        }
    }
}

/// A decoded RISC-V instruction structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionFormat {
    RType { funct7: u32, rs2: Register, rs1: Register, funct3: u32, rd: Register, opcode: u32 },
    IType { imm: u32, rs1: Register, funct3: u32, rd: Register, opcode: u32 },
    SType { imm: u32, rs2: Register, rs1: Register, funct3: u32, opcode: u32 },
    BType { imm: u32, rs2: Register, rs1: Register, funct3: u32, opcode: u32 },
    UType { imm: u32, rd: Register, opcode: u32 },
    JType { imm: u32, rd: Register, opcode: u32 },
    Ecall,
}

/// A single clock cycle represented in the zkVM trace.
/// The STARK prover will consume this 2D matrix directly.
#[derive(Debug, Clone)]
pub struct ExecutionStep {
    pub clk: u32,
    pub pc: u32,
    pub decoded_instruction: InstructionFormat,
    pub registers: [u32; 32], // Full RV32 state
    pub memory_reads: Vec<(u32, u32)>, // (Address, Value)
    pub memory_writes: Vec<(u32, u32)>, // (Address, Value)
}

/// The entire execution history of an ELF binary payload.
#[derive(Debug, Default, Clone)]
pub struct ExecutionTrace {
    pub steps: Vec<ExecutionStep>,
    pub public_inputs: Vec<u32>,
    pub private_inputs: Vec<u32>,
}
