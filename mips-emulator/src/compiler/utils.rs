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

pub fn get_register_alias(reg_string: &str) -> &str {
    return match reg_string {
        "0" => "r0",
        "1" => "at",
        "2" => "v0",
        "3" => "v1",
        "4" => "a0",
        "5" => "a1",
        "6" => "a2",
        "7" => "a3",
        "8" => "t0",
        "9" => "t1",
        "10" => "t2",
        "11" => "t3",
        "12" => "t4",
        "13" => "t5",
        "14" => "t6",
        "15" => "t7",
        "16" => "s0",
        "17" => "s1",
        "18" => "s2",
        "19" => "s3",
        "20" => "s4",
        "21" => "s5",
        "22" => "s6",
        "23" => "s7",
        "24" => "t8",
        "25" => "t9",
        "26" => "k0",
        "27" => "k1",
        "28" => "gp",
        "29" => "sp",
        "30" => "fp",
        "31" => "ra",
        _ => reg_string
    };
    
}

pub fn decode_immediate(imm_string: &str) -> i16{
    if imm_string.starts_with("0x"){
        let res = i16::from_str_radix(imm_string.strip_prefix("0x").unwrap(), 16);
        return match res {
            Ok(ok) => ok,
            Err(_err) => {println!("errore"); return 0;}
        }
    }
    else{
        return match imm_string.parse::<i16>() {
            Ok(ok) => ok,
            Err(_err) => {println!("errore"); return 0;}
        }
    }
}

pub fn cast_immediate(im: i16) -> u32 {
    (im as u32) & 0x0000ffff
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

pub fn _get_hex(value: u32) -> String{
    let aa = format!("{value:08x}");
    return aa;
}

pub fn _get_binary(value: u32) -> String{
    let aa = format!("{value:032b}");
    return aa;
}

pub fn _get_binary_i16(value: i16) -> String{
    let aa = format!("{value:016b}");
    return aa;
}

pub fn _get_binary_u16(value: u16) -> String{
    let aa = format!("{value:016b}");
    return aa;
}

#[cfg(test)]
mod tests {

    use std::i16;

    use super::*;

    #[test]
    fn immediate_parse_test(){
        assert_eq!(decode_immediate("533"), 533);
        assert_eq!(decode_immediate("4242"), 4242);
        assert_eq!(decode_immediate("0"), 0);
        assert_eq!(decode_immediate("0x4fd2"), 0x4fd2);
        assert_eq!(decode_immediate("0x12C5"), 0x12C5);
        assert_eq!(decode_immediate("0x0001"), 0x0001);
        assert_eq!(decode_immediate("0x2D1F"), 0x2D1F);
        assert_eq!(decode_immediate("0x3D1A"), 0x3D1A);
        assert_eq!(decode_immediate("-44"), -44);
    }

    #[test]
    fn immediate_cast(){
        let a: i16 = decode_immediate("0x221f");
        let b: u32 = (a as u32) & 0x0000ffff;
        println!("{}\t\t{}", a, _get_binary_i16(a));
        println!("{}\t\t{}", b, _get_binary(b));
        assert_eq!(a, 0x221f);
        //assert_eq!(b, 0);
    }
}
