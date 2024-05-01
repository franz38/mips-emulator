use super::slice_instruction::{get_rt, get_op, get_immediate, get_rs};


pub const ADDI: u32 = 8;
pub const BEQ: u32 = 4;
pub const BNE: u32 = 5;
pub const _BLT: u32 = 5;
pub const _BLE: u32 = 5;
pub const SW: u32 = 43;
pub const LW: u32 = 35;


pub fn run_i(instruction: u32, registers: &mut [i32], virtual_pc: &mut usize, memory: &mut [i32]) -> bool{
    let rs = get_rs(instruction);
    let rt = get_rt(instruction);
    let im = get_immediate(instruction);
    let op = get_op(instruction);

    match op {
        ADDI => addi(registers, rs, rt, im),
        BEQ => beq(registers, rs, rt, im, virtual_pc),
        BNE => bne(registers, rs, rt, im, virtual_pc),
        // BLT => blt(registers, rs, rt, im, virtual_pc),
        // BLE => ble(registers, rs, rt, im, virtual_pc),
        LW => lw(registers, rs, rt, im, memory),
        SW => sw(registers, rs, rt, im, memory),
        _ => return true
    }

    return false;
}

fn addi(registers: &mut [i32], rs: u32, rt: u32, im: i32){
    registers[rt as usize] = registers[rs as usize] + im;
}

// branch on equal
fn beq(registers: &mut [i32], rs: u32, rt: u32, im: i32, virtual_pc: &mut usize){
    if registers[rs as usize] - registers[rt as usize] == 0{
        *virtual_pc += im as usize;
    }
}

// branch on not equal
fn bne(registers: &mut [i32], rs: u32, rt: u32, im: i32, virtual_pc: &mut usize){
    if registers[rs as usize] - registers[rt as usize] != 0{
        *virtual_pc += im as usize;
    }
}

// branch on less than
fn _blt(registers: &mut [i32], rs: u32, rt: u32, im: i32, virtual_pc: &mut usize){
    if registers[rs as usize] < registers[rt as usize]{
        *virtual_pc += (im/4)as usize;
    }
}

// branch on less than or equal
fn _ble(registers: &mut [i32], rs: u32, rt: u32, im: i32, virtual_pc: &mut usize){
    if registers[rs as usize] <= registers[rt as usize]{
        *virtual_pc += (im/4)as usize;
    }
}

fn sw(registers: &mut [i32], rs: u32, rt: u32, im: i32, memory: &mut [i32]){
    memory[(rs as i32 + im) as usize] = registers[rt as usize];
}

fn lw(registers: &mut [i32], rs: u32, rt: u32, im: i32, memory: &mut [i32]){
    registers[rt as usize] = memory[(rs as i32 + im) as usize];
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn addi_test(){
//         let mut registers: [u32; 32] = [1; 32];
//         addi(&mut registers, 1, 2, 200);
//         assert_eq!(registers[1], 201);
//         addi(&mut registers, 1, 1, 20);
//         assert_eq!(registers[1], 221);
//         addi(&mut registers, 2, 1, 2);
//         assert_eq!(registers[2], 223);
//     }

// }
