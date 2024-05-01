pub mod utils;
mod encode_r;
mod encode_i;
mod encode_j;


//      |   6    |   5  |   5   |   5   |   5   |    6      |
// R:   | opcode | rs   | rt    | rd    | shamt | funct     |
// I:   | opcode | rs   | rt    | immediate                 |
// J:   | opcode | address                                  |


pub fn compile_code(lines: Vec<String>) -> [i32; 30]{
    let size: usize = lines.len();
    let mut binary_code: [i32; 30] = [0; 30];
    for i in 0..size{
        binary_code[i] = compile_instruction(lines[i].clone()) as i32;
    }
    return binary_code;
}

pub fn compile_instruction(line: String) -> u32{
    let cleaned = line.replace(",", "");
    let split: Vec<&str> = cleaned.split(" ").collect();

    let instruction: u32 = match split[0].to_uppercase().as_str() {
        "ADD" | "AND" | "SUB" => encode_r::encode_r(split),
        "MFLO" | "MFHI" => encode_r::encode_mf(split),
        "MULT" | "DIV" => encode_r::encode_mult(split),
        "ADDI" => encode_i::encode_addi(split),
        "SW" | "LW" => encode_i::encode_mem(split),
        "BEQ" | "BNE" => encode_i::encode_bne(split),
        "SLL" | "SRL" => encode_r::encode_logical_shift(split),
        "J" | "JAL" => encode_j::encode_jump(split),
        _ => 0
    };

    return instruction;
}



#[cfg(test)]
mod test_instruction_encoding{
    use crate::compiler::compile_instruction;


    #[test]
    fn add_encode(){
        let encoded = compile_instruction(String::from("ADD $31 $19 $2"));
        assert_eq!(encoded, 0x0262F820);
    }

    #[test]
    fn sub_encode(){
        let encoded = compile_instruction(String::from("SUB $3 $9 $21"));
        assert_eq!(encoded, 0x01351822);
    }

    #[test]
    fn and_encode(){
        let encoded = compile_instruction(String::from("AND $15 $1 $12"));
        assert_eq!(encoded, 0x002C7824);
    }

    #[test]
    fn mult_encode(){
        let encoded = compile_instruction(String::from("MULT $2 $3"));
        assert_eq!(encoded, 0x00430018);
    }

    #[test]
    fn mflo_encode(){
        let encoded = compile_instruction(String::from("MFLO $1"));
        assert_eq!(encoded, 0x00000812);
    }

    #[test]
    fn mfhi_encode(){
        let encoded = compile_instruction(String::from("MFHI $10"));
        assert_eq!(encoded, 0x00005010);
    }

    #[test]
    fn div_encode(){
        let encoded = compile_instruction(String::from("DIV $1 $2"));
        assert_eq!(encoded, 0x0022001A);
    }

    #[test]
    fn addi_encode(){
        let encoded = compile_instruction(String::from("ADDI $2 $3 543"));
        assert_eq!(encoded, 0x2062021F);
    }

    #[test]
    fn beq_encode(){
        let encoded = compile_instruction(String::from("BEQ $2 $3 200"));
        assert_eq!(encoded, 0x104300C8);
    }

    #[test]
    fn bne_encode(){
        let encoded = compile_instruction(String::from("BNE $2 $3 200"));
        assert_eq!(encoded, 0x144300C8);
    }

    #[test]
    fn sll_encode(){
        let encoded = compile_instruction(String::from("SLL $t1 $t2 10"));
        assert_eq!(encoded, 0x000A4A80);
    }

    #[test]
    fn srl_encode(){
        let encoded = compile_instruction(String::from("SRL $t3 $t1 4"));
        assert_eq!(encoded, 0x00095902);
    }

    #[test]
    fn j_encode(){
        let encoded = compile_instruction(String::from("J 0x1F1A"));
        assert_eq!(encoded, 0x08001F1A);
    }

    #[test]
    fn jal_encode(){
        let encoded = compile_instruction(String::from("JAL 0x1F1A"));
        assert_eq!(encoded, 0x0C001F1A);
    }

    #[test]
    fn sw_encode(){
        let encoded = compile_instruction(String::from("SW $2 0xF4($1)"));
        assert_eq!(encoded, 0xAC2200F4);
    }

    #[test]
    fn lw_encode(){
        let encoded = compile_instruction(String::from("LW $4 0x24F1($2)"));
        assert_eq!(encoded, 0x8C4424F1);
    }

}


#[cfg(test)]
mod test_instruction_parsing {
    use crate::compiler::compile_instruction;


    #[test]
    fn register_alias_test(){
        let mut encoded ;
        encoded = compile_instruction(String::from("ADD $t1 $t2 $t3"));
        assert_eq!(encoded, 0x014B4820);
        encoded = compile_instruction(String::from("AND $t2 $zero $a2"));
        assert_eq!(encoded, 0x00065024);
        encoded = compile_instruction(String::from("SUB $t7 $zero $a3"));
        assert_eq!(encoded, 0x00077822);
        encoded = compile_instruction(String::from("MULT $t2 $3"));
        assert_eq!(encoded, 0x01430018);
        encoded = compile_instruction(String::from("ADDI $t3 $r0 543"));
        assert_eq!(encoded, 0x200B021F);
    }

    #[test]
    fn hex_immediate_test(){
        let mut encoded ;
        encoded = compile_instruction(String::from("ADDI $t3 $0 0x221F"));
        assert_eq!(encoded, 0x200B221F);
        encoded = compile_instruction(String::from("BEQ $t3 $0 0x3D1A"));
        assert_eq!(encoded, 0x11603D1A);
        encoded = compile_instruction(String::from("BNE $2 $3 200"));
        assert_eq!(encoded, 0x144300C8);
    }

    #[test]
    fn negative_immediate_test(){
        let encoded ;
        encoded = compile_instruction(String::from("ADDI $t0 $0 -44"));
        assert_eq!(encoded, 0x2008ffd4);
    }

    #[test]
    fn case_insensitive_test(){
        assert_eq!(
            compile_instruction(String::from("ADDI $t3 $0 0xD21F")), 
            compile_instruction(String::from("addi $t3 $0 0xD21F"))
        );
        assert_eq!(
            compile_instruction(String::from("BEQ $2 $3 200")), 
            compile_instruction(String::from("beq $2 $3 200"))
        );
        assert_eq!(
            compile_instruction(String::from("AND $15 $1 $12")), 
            compile_instruction(String::from("aNd $15 $1 $12"))
        );
    }

    #[test]
    fn remove_semicolon(){
        assert_eq!(
            compile_instruction(String::from("ADDI $t3 $0 0xD21F")), 
            compile_instruction(String::from("ADDI $t3, $0, 0xD21F"))
        );
        assert_eq!(
            compile_instruction(String::from("SUB $t3 $1 $t3")), 
            compile_instruction(String::from("SUB $t3, $1, $t3"))
        );
    }

}
