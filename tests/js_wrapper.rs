use mips_emulator::compile_and_execute_js;


#[test]
fn addi() {
    let mut v_state: Vec<u32> = compile_and_execute_js(String::from("ADDI $1 $0 22"), None, None);
    assert_eq!(v_state[4], 0);
    assert_eq!(v_state[5], 22);
    
    v_state = compile_and_execute_js(String::from("ADDI $1 $0 22\nADDI $2 $0 33"), None, None);
    assert_eq!(v_state[4], 0);
    assert_eq!(v_state[5], 22);
    assert_eq!(v_state[6], 33);
}
