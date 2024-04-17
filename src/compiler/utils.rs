pub fn get_op_code(op_string: &str) -> u32 {
    match op_string.to_uppercase().as_str() {
        "ADD" | "AND" | "SUB" | "MULT" | "SLL" | "SRL" | "MFLO" | "MFHI" | "DIV" => return 0,
        "ADDI" => return 8,
        "SW" => return 43,
        "LW" => return 35,
        "BEQ" => return 4,
        "BNE" => return 5,
        "J" => return 2,
        "JAL" => return 3,
        _ => return 0,
    }
}

pub fn get_func_code(command_string: &str) -> u32{
    match command_string.to_uppercase().as_str() {
        "ADD" => return 32,
        "AND" => return 36,
        "SUB" => return 34,
        "MULT" => return 24,
        "MFLO" => return 18,
        "MFHI" => return 16,
        "DIV" => return  26,
        "SLL" => return 0,
        "SRL" => return 2,
        _ => return  0
    }
}

pub fn convert_register_alias(reg_alias: &str) -> &str{
    return match reg_alias {
        "r0" | "zero" => "0",
        "at" => "1",
        "v0" => "2",
        "v1" => "3",
        "a0" => "4",
        "a1" => "5",
        "a2" => "6",
        "a3" => "7",
        "t0" => "8",
        "t1" => "9",
        "t2" => "10",
        "t3" => "11",
        "t4" => "12",
        "t5" => "13",
        "t6" => "14",
        "t7" => "15",
        "s0" => "16",
        "s1" => "17",
        "s2" => "18",
        "s3" => "19",
        "s4" => "20",
        "s5" => "21",
        "s6" => "22",
        "s7" => "23",
        "t8" => "24",
        "t9" => "25",
        "k0" => "26",
        "k1" => "27",
        "gp" => "28",
        "sp" => "29",
        "fp" => "30",
        "ra" => "31",
        _ => reg_alias
    };
}

pub fn decode_immediate(imm_string: &str) -> u32{
    if imm_string.starts_with("0x"){
        let res = u32::from_str_radix(imm_string.strip_prefix("0x").unwrap(), 16);
        return match res {
            Ok(ok) => ok,
            Err(_err) => {println!("errore"); return 0;}
        }
    }
    else{
        return match imm_string.parse::<u32>() {
            Ok(ok) => ok,
            Err(_err) => {println!("errore"); return 0;}
        }
    }
}

pub fn decode_register_number(reg_string: &str) -> u32{
    let mut s = reg_string.chars();
    s.next();
    let sliced_name = s.as_str();

    let parsed = sliced_name.parse::<u32>();
    match parsed {
        Ok(ok) => {return ok},
        Err(_err) => {
            let converted = convert_register_alias(sliced_name);
            let parsed2 = converted.parse::<u32>();
            match parsed2 {
                Ok(ok) => {return ok},
                Err(_err) => {}
            }
            println!("invalid register"); 
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn immediate_parse_test(){
        assert_eq!(decode_immediate("533"), 533);
        assert_eq!(decode_immediate("42442"), 42442);
        assert_eq!(decode_immediate("0"), 0);
        assert_eq!(decode_immediate("0xf132"), 0xf132);
        assert_eq!(decode_immediate("0xC812C5"), 0xC812C5);
        assert_eq!(decode_immediate("0x00001"), 0x00001);
        assert_eq!(decode_immediate("0xD21F"), 0xD21F);
        assert_eq!(decode_immediate("0x3D1A"), 0x3D1A);
    }

}