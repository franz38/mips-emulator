use super::slice_instruction::{get_op, get_jump};


pub const J: u32 = 2;
pub const JAL: u32 = 3;

pub fn run_j(instruction: u32, registers: &mut [i32], virtual_pc: &mut usize) -> bool{
    let op = get_op(instruction);
    let jump = get_jump(instruction);

    match op {
        J => j(registers, jump, virtual_pc),
        JAL => jal(registers, jump, virtual_pc),
        _ => return true
    }

    return false;
}

fn j(_registers: &mut [i32], jump: u32, virtual_pc: &mut usize){
    *virtual_pc += jump as usize;
}

fn jal(registers: &mut [i32], jump: u32, virtual_pc: &mut usize){
    registers[31] = (*virtual_pc * 4) as i32; // $ra = PC
    *virtual_pc += jump as usize;
}
