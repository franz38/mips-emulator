use mips_emulator::compile_and_execute;


#[test]
fn step1() {
    let v_state = compile_and_execute(String::from("ADDI $1 $0 11\nADDI $2 $0 $22\nADD $3 $2 $1"), None, Some(1));
    assert_eq!(v_state.registers[1], 11);
    assert_eq!(v_state.registers[2], 0);
    assert_eq!(v_state.registers[3], 0);
    assert_eq!(v_state.pc, 4);
    
    let v_state = compile_and_execute(String::from("ADDI $1 $0 11\nADDI $2 $0 $22\nADD $3 $2 $1"), None, Some(2));
    assert_eq!(v_state.registers[1], 11);
    assert_eq!(v_state.registers[2], 22);
    assert_eq!(v_state.registers[3], 0);
    assert_eq!(v_state.pc, 8);

    let v_state = compile_and_execute(String::from("ADDI $1 $0 11\nADDI $2 $0 $22\nADD $3 $2 $1"), None, Some(3));
    assert_eq!(v_state.registers[1], 11);
    assert_eq!(v_state.registers[2], 22);
    assert_eq!(v_state.registers[3], 33);
    assert_eq!(v_state.pc, 12);
}


