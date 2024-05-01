use super::utils::{decode_immediate, decode_register_number, get_func_code, get_op_code};


pub fn encode_r(split: Vec<&str>) -> u32{
    let op_code = get_op_code(split[0]);
    let rs: u32 = decode_register_number(split[2]);
    let rt: u32 = decode_register_number(split[3]);
    let rd: u32 = decode_register_number(split[1]);
    let sa: u32 = 0;
    let func: u32 = get_func_code(split[0]);
    return ((op_code << 26) + (rs << 21) + (rt << 16) + (rd << 11) + (sa << 6) + func) as u32;
}

pub fn encode_mult(split: Vec<&str>) -> u32{
    let op_code = get_op_code(split[0]);
    let rs: u32 = decode_register_number(split[1]);
    let rt: u32 = decode_register_number(split[2]);
    let rd: u32 = 0;
    let sa: u32 = 0;
    let func: u32 = get_func_code(split[0]);
    return ((op_code << 26) + (rs << 21) + (rt << 16) + (rd << 11) + (sa << 6) + func) as u32;
}

pub fn encode_mf(split: Vec<&str>) -> u32{
    let op_code = get_op_code(split[0]);
    let rs: u32 = 0;
    let rt: u32 = 0;
    let rd: u32 = decode_register_number(split[1]);
    let sa: u32 = 0;
    let func: u32 = get_func_code(split[0]);
    return ((op_code << 26) + (rs << 21) + (rt << 16) + (rd << 11) + (sa << 6) + func) as u32;
}

pub fn encode_logical_shift(split: Vec<&str>) -> u32{
    let op_code = get_op_code(split[0]);
    let rs: u32 = 0;
    let rt: u32 = decode_register_number(split[2]);
    let rd: u32 = decode_register_number(split[1]);
    let sa: u32 = decode_immediate(split[3]) as u32;
    let func: u32 = get_func_code(split[0]);
    return ((op_code << 26) + (rs << 21) + (rt << 16) + (rd << 11) + (sa << 6) + func) as u32;
}
