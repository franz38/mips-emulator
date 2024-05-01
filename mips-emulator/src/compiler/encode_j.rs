use super::utils::{decode_immediate, get_op_code};


pub fn encode_jump(split: Vec<&str>) -> u32{
    let op_code: u32 = get_op_code(split[0]);
    let address: u32 = decode_immediate(split[1]) as u32;
    return ((op_code << 26) + address) as u32;
}
