use std::{u32, usize};

use crate::state::{StateManager, INSTRUCTION_SIZE, REG_SIZE};

mod run_i;
mod run_r;
mod run_j;
mod slice_instruction;


pub fn run(mut v: Vec<u32>, steps: Option<u32>) -> Vec<u32> {
   
    let steps_number = match steps {
        Some(v) => v,
        None => u32::max_value()
    };

    let (registers, instructions, memory) = StateManager::split(&mut v);
    
    let mut virtual_pc: usize = (registers[32]/4) as usize;
    let mut lo = registers[34];
    let mut hi = registers[35];
   
    let mut exit: bool = false;
    let mut steps_counter = 0;

    while virtual_pc < INSTRUCTION_SIZE && !exit {
        
        registers[33] = instructions[virtual_pc]; // instruction register update
        
        if registers[33] == 0 {
            exit = true;
            continue;
        }

        virtual_pc += 1;
        
        exit = match slice_instruction::get_op(registers[33]) {
            0 => run_r::run_r(registers[33], registers, &mut lo, &mut hi),
            2 | 3 => run_j::run_j(registers[33], registers, &mut virtual_pc),
            _ => run_i::run_i(registers[33], registers, &mut virtual_pc, memory)
        };

        steps_counter += 1;
        if steps_counter >= steps_number {
            exit = true;
        }

    }

    registers[32] = (virtual_pc*4) as u32;

    return v;

}

