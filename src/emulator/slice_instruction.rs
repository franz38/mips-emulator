pub const OP_MASK: u32 = 63 << 26;
pub const RS_MASK: u32 = 31 << 21;
pub const RT_MASK: u32 = 31 << 16;
pub const RD_MASK: u32 = 31 << 11;
pub const SA_MASK: u32 = 31 << 6;
pub const FUNC_MASK: u32 = 63;
pub const IMM_MASK: u32 = 65535;
pub const JUMP_MASK: u32 = 0x3FFFFFF;

pub fn get_op(instruction: u32) -> u32 {
    return (instruction & OP_MASK) >> 26;
}

pub fn get_rs(instruction: u32) -> u32 {
    return (instruction & RS_MASK) >> 21;
}

pub fn get_rt(instruction: u32) -> u32 {
    return (instruction & RT_MASK) >> 16;
}

pub fn get_rd(instruction: u32) -> u32 {
    return (instruction & RD_MASK) >> 11;
}

pub fn get_sh(instruction: u32) -> u32 {
    return (instruction & SA_MASK) >> 6;
}

pub fn get_function(instruction: u32) -> u32 {
    return instruction & FUNC_MASK;
}

pub fn get_immediate(instruction: u32) -> u32 {
    return instruction & IMM_MASK;
}

pub fn get_jump(instruction: u32) -> u32 {
    return instruction & JUMP_MASK;
}
