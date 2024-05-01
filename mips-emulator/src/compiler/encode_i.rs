use super::utils::{cast_immediate, decode_immediate, decode_register_number, get_op_code};


pub fn encode_addi(split: Vec<&str>) -> u32{
    let op_code = get_op_code(split[0]);
    let rs = decode_register_number(split[2]);
    let rt = decode_register_number(split[1]);
    let im = cast_immediate(decode_immediate(split[3]));
    return ((op_code << 26) + (rs << 21) + (rt << 16) + im) as u32;
}

pub fn encode_bne(split: Vec<&str>) -> u32{
    let op_code: u32 = get_op_code(split[0]);
    let rs = decode_register_number(split[1]);
    let rt = decode_register_number(split[2]);
    let im = cast_immediate(decode_immediate(split[3]));
    return ((op_code << 26) + (rs << 21) + (rt << 16) + im) as u32;
}

pub fn encode_mem(split: Vec<&str>) -> u32{
    let op_code = get_op_code(split[0]);
    let rt = decode_register_number(split[1]);
    let cleaned = split[2].replace(")", "");
    let tmp: Vec<&str> = cleaned.split("(").collect();
    let rs = decode_register_number(tmp[1]);
    let im = cast_immediate(decode_immediate(tmp[0]));
    return ((op_code << 26) + (rs << 21) + (rt << 16) + im) as u32;
}
