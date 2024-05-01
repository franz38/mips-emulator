use mips_emulator::{compile, get_state, init, run, state::{INSTRUCTION_SIZE, REG_SIZE}};


const PROGRAM1: &str = "ADDI $t0 $0 5\nADDI $t1 $t0 10\nADD $t2 $t1 $t0\nsw $t2 0($zero)\nlw $t3 0($zero)";
const PROGRAM2: &str = "ADDI $t0 $0 -5\nADDI $t1 $t0 -10\nADD $t2 $t1 $t0\nsw $t2 0($zero)\nlw $t3 0($zero)";

#[test]
fn one_line_program() {
    let ptr = init();
    compile(ptr, String::from("ADDI $t0 $0 27"));
    let mut state = get_state(ptr);
    assert_eq!(state[32], 0);
    run(ptr, None);
    state = get_state(ptr);
    assert_eq!(state[32], 4);
    assert_eq!(state[8], 27);
}

#[test]
fn another_one_line_program() {
    let ptr = init();
    compile(ptr, String::from("ADDI $t0 $0 -27"));
    let mut state = get_state(ptr);
    assert_eq!(state[32], 0);
    run(ptr, None);
    state = get_state(ptr);
    assert_eq!(state[32], 4);
    assert_eq!(state[8], -27);
}

#[test]
fn compile_short_program1() {
    let ptr = init();
    compile(ptr, String::from(PROGRAM1));
    let state = get_state(ptr);
    assert_eq!(state[REG_SIZE], 0x20080005);
    assert_eq!(state[REG_SIZE+1], 0x2109000A);
    assert_eq!(state[REG_SIZE+2], 0x01285020);
    assert_eq!(state[REG_SIZE+3] as u32, 0xAC0A0000);
    assert_eq!(state[REG_SIZE+4] as u32, 0x8C0B0000);
}

#[test]
fn execute_short_program() {
    let ptr = init();
    compile(ptr, String::from(PROGRAM1));
    run(ptr, None);
    let state = get_state(ptr);
    assert_eq!(state[8], 5);
    assert_eq!(state[9], 15);
    assert_eq!(state[10], 20);
    assert_eq!(state[REG_SIZE + INSTRUCTION_SIZE], 20);
    assert_eq!(state[11], 20);
}

#[test]
fn after_program_execution_pc_iscorrect() {
    let ptr = init();
    compile(ptr, String::from(PROGRAM1));
    run(ptr, None);
    let state = get_state(ptr);
    assert_eq!(state[32], 20); 
}

#[test]
fn execute_short_program_step_by_step() {
    let ptr = init();
    compile(ptr, String::from(PROGRAM1));
    run(ptr, Some(1));
    let mut state = get_state(ptr);
    assert_eq!(state[8], 5);
    assert_eq!(state[9], 0);
    assert_eq!(state[10], 0);
    assert_eq!(state[REG_SIZE + INSTRUCTION_SIZE], 0);
    assert_eq!(state[11], 0);
    assert_eq!(state[32], 4);
    assert_eq!(state[33], 0x20080005);
    
    run(ptr, Some(1));
    state = get_state(ptr);
    assert_eq!(state[32], 8);
    assert_eq!(state[9], 15);
    assert_eq!(state[33], 0x2109000a);
}

#[test]
fn compile_short_program2() {
    let ptr = init();
    compile(ptr, String::from(PROGRAM2));
    let state = get_state(ptr);
    assert_eq!(state[REG_SIZE], 0x2008fffb);
    assert_eq!(state[REG_SIZE+1], 0x2109fff6);
    assert_eq!(state[REG_SIZE+2], 0x01285020);
    assert_eq!(state[REG_SIZE+3] as u32, 0xAC0A0000);
    assert_eq!(state[REG_SIZE+4] as u32, 0x8C0B0000);
}


#[test]
fn execute_short_program2() {
    let ptr = init();
    compile(ptr, String::from(PROGRAM2));
    run(ptr, None);
    let state = get_state(ptr);
    assert_eq!(state[8], -5);
    assert_eq!(state[9], -15);
    assert_eq!(state[10], -20);
    assert_eq!(state[REG_SIZE + INSTRUCTION_SIZE], -20);
    assert_eq!(state[11], -20);
}
