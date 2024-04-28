use std::{fmt::format, usize};

use crate::compiler::utils::get_register_alias;

pub const REG_SIZE: usize = 36;
pub const INSTRUCTION_SIZE: usize = 1024;
pub const STACK_SIZE: usize = 4096;

pub const A: usize = 0;
pub const B: usize = REG_SIZE;
pub const C: usize = REG_SIZE + INSTRUCTION_SIZE;
pub const D: usize = REG_SIZE + INSTRUCTION_SIZE + STACK_SIZE;

// #[wasm_bindgen]
pub struct State{
    pub registers: [u32; 32],
    pub memory: [u32; 200],
    pub lo: u32,
    pub hi: u32,
    pub pc: u32,
    pub ir: u32
}

// registers : [pc, ir, lo, hi, $0, $1, .., $32]

pub struct StateManager {}

impl StateManager {
    
    pub fn init() -> Vec<u32> {
        let mut v: Vec<u32> = vec![0; D];
        return v;
    }

    pub fn get(ptr: *mut u32) -> Vec<u32> {
        unsafe {
            let mut v = Vec::from_raw_parts(ptr, D, D);
            return v;
        }
    }

    pub fn compile(ptr: *mut u32, code_binary: [u32; 30]) {
        let mut v = Self::get(ptr);
        
        for i in 0..v.len(){
            v[i] = 0;
        }

        for (pos, instruction) in code_binary.iter().enumerate(){
            v[REG_SIZE + pos] = *instruction; 
        }

        std::mem::forget(v);
    }

    pub fn split(v: &mut [u32]) -> (&mut [u32], &mut [u32], &mut [u32]) {
        let (registers, b) = v.split_at_mut(REG_SIZE);
        let (instructions, memory) = b.split_at_mut(INSTRUCTION_SIZE);

        return (registers, instructions, memory)
    }

    pub fn get_pc(v: &[u32]) -> u32 { v[32] }

    pub fn get_ir(v: &[u32]) -> u32 { v[33] }
    
    pub fn get_lo(v: &[u32]) -> u32 { v[34] }
    
    pub fn get_hi(v: &[u32]) -> u32 { v[35] }

    pub fn to_string(v: &[u32]) -> String {
        let r: String = v[A..B].iter().map(|v| v.to_string() + "  ").collect();
        let instructions: String = v[B..(B+100)].iter().map(|v| v.to_string() + "\t").collect();
        let memory: String = v[C..(C+100)].iter().map(|v| v.to_string() + "\t").collect();
        return format!("registers:\n{}\n\ninstructions:\n{}\n\nmemory:\n{}", r, instructions, memory);
    }
}

