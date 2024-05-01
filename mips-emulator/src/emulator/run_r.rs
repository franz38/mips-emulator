use super::slice_instruction::{get_rt, get_function, get_rd, get_rs, get_sh};


pub const ADD: u32 = 32;
pub const SUB: u32 = 34;
pub const MULT: u32 = 24;
pub const AND: u32 = 36;
pub const OR: u32 = 37;
pub const SLL: u32 = 0;
pub const SRL: u32 = 2;
pub const MFLO: u32 = 18;
pub const MFHI: u32 = 16;

pub fn run_r(instruction: u32, registers: &mut [i32], lo_p: &mut u32, hi_p: &mut u32) -> bool{
    let rs = get_rs(instruction);
    let rt = get_rt(instruction);
    let rd = get_rd(instruction);
    let sh: u32 = get_sh(instruction);
    let function = get_function(instruction);

    match function {
        ADD => add(registers, rs, rt, rd),
        SUB => sub(registers, rs, rt, rd),
        MULT => mult(registers, rs, rt, lo_p, hi_p),
        AND => and(registers, rs, rt, rd),
        OR => or(registers, rs, rt, rd),
        SLL => sll(registers, rt, rd, sh),
        SRL => srl(registers, rt, rd, sh),
        MFLO => mflo(registers, rd, lo_p),
        MFHI => mfhi(registers, rd, hi_p),
        _ => return true
    }

    return false;
}

fn add(registers: &mut [i32], rs: u32, rt: u32, rd: u32){
    registers[rd as usize] = registers[rs as usize] + registers[rt as usize];
}

fn sub(registers: &mut [i32], rs: u32, rt: u32, rd: u32){
    registers[rd as usize] = registers[rs as usize] - registers[rt as usize];
}

fn and(registers: &mut [i32], rs: u32, rt: u32, rd: u32){
    registers[rd as usize] = registers[rs as usize] & registers[rt as usize];
}

fn or(registers: &mut [i32], rs: u32, rt: u32, rd: u32){
    registers[rd as usize] = registers[rs as usize] | registers[rt as usize];
}

fn sll(registers: &mut [i32], rt: u32, rd: u32, sh: u32){
    registers[rd as usize] = registers[rt as usize] << sh;
}

fn srl(registers: &mut [i32], rt: u32, rd: u32, sh: u32){
    registers[rd as usize] = registers[rt as usize] >> sh;
}

fn _nor(registers: &mut [i32], rs: u32, rt: u32, rd: u32){
    registers[rd as usize] = !(registers[rs as usize] | registers[rt as usize]);
}

fn _xor(registers: &mut [i32], rs: u32, rt: u32, rd: u32){
    registers[rd as usize] = registers[rs as usize] ^ registers[rt as usize];
}

fn mult(registers: &mut [i32], rs: u32, rt: u32, lo_p: &mut u32, hi_p: &mut u32){
    let tot: u64 = registers[rs as usize] as u64 * registers[rt as usize] as u64;
    *lo_p = (tot & 0xFFFFFFFF) as u32;
    *hi_p = (tot >> 32) as u32;
}

fn mflo(registers: &mut [i32], rd: u32, lo_p: &mut u32){
    registers[rd as usize] = *lo_p as i32;
}

fn mfhi(registers: &mut [i32], rd: u32, hi_p: &mut u32){
    registers[rd as usize] = *hi_p as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_instructions(){
        let mut registers: [i32; 32] = [1; 32];
        add(&mut registers, 1, 2, 2);
        assert_eq!(registers[2], 2);
        sub(&mut registers, 2, 2, 3);
        assert_eq!(registers[3], 0);
    }

    #[test]
    fn and_test(){
        let mut registers: [i32; 32] = [1; 32];
        registers[1] = 534;
        registers[2] = 53434;
        and(&mut registers, 1, 2, 3);
        assert_eq!(registers[3], 18);
        registers[4] = 324;
        registers[5] = 869;
        and(&mut registers, 4, 5, 6);
        assert_eq!(registers[6], 324);
    }

    #[test]
    fn or_test(){
        let mut registers: [i32; 32] = [1; 32];
        registers[2] = 534;
        registers[3] = 53434;
        or(&mut registers, 2, 3, 3);
        assert_eq!(registers[3], 53950);
        registers[4] = 324;
        registers[5] = 869;
        or(&mut registers, 4, 5, 1);
        assert_eq!(registers[1], 869);
    }
}
