use super::{slice_instruction::{get_op, get_rt, get_rs, get_rd, get_immediate, get_function}, run_i::{ADDI, BEQ, BNE}, run_r::{ADD, SUB, MULT}};


pub fn decode(instruction: u32) -> String {
    let op = get_op(instruction);
    if op == 0 {
        // R
        return decode_r(instruction);
    }
    // I
    return decode_i(instruction);
}

fn get_register_name(register_code: u32) -> String {
    return String::from("$") + &register_code.to_string();
}

fn decode_r(instruction: u32) -> String {
    let rs = get_rs(instruction);
    let rt = get_rt(instruction);
    let rd = get_rd(instruction);
    let function = get_function(instruction);

    let op = match function {
        ADD => "ADD",
        SUB => "SUB",
        MULT => "MULT",
        _ => "",
    };

    let r1 = get_register_name(rs);
    let r2 = get_register_name(rt);
    let r3 = get_register_name(rd);

    return format!("{} {} {} {}", op, r1.as_str(), r2.as_str(), r3.as_str());
}

fn decode_i(instruction: u32) -> String {
    let op = get_op(instruction);
    let rs = get_rs(instruction);
    let rt = get_rt(instruction);
    let im = get_immediate(instruction);

    let op_string = match op {
        ADDI => "ADDI",
        BEQ => "BEQ",
        BNE => "BNE",
        _ => "",
    };

    let r1 = get_register_name(rs);
    let r2 = get_register_name(rt);

    return format!(
        "{} {} {} {}",
        op_string,
        r1.as_str(),
        r2.as_str(),
        im.to_string()
    );
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn decode_r_test() {
//         let instruction: u32 = 0x0262F820;
//         let decoded = decode_r(instruction);
//         assert_eq!(decoded, "ADD $31 $19 $2");
//     }
// }
