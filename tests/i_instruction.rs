use mips_emulator::{compile_and_execute, state::State};

#[test]
fn addi() {
    let mut registers: [u32; 32];
    registers = compile_and_execute(String::from("ADDI $1 $0 22"), None).registers;
    assert_eq!(registers[0], 0);
    assert_eq!(registers[1], 22);
    registers = compile_and_execute(String::from("ADDI $1 $0 22\nADDI $2 $0 33"), None).registers;
    assert_eq!(registers[0], 0);
    assert_eq!(registers[1], 22);
    assert_eq!(registers[2], 33);

    registers =
        compile_and_execute(String::from("ADDI $1 $0 22"), Some(State::blank_state())).registers;
    assert_eq!(registers[0], 0);
    assert_eq!(registers[1], 22);
}

#[test]
fn beq() {
    let mut prev_state = State::blank_state();
    prev_state.registers[1] = 2;
    prev_state.registers[2] = 2;
    let state = compile_and_execute(String::from("BEQ $1 $2 10"), Some(prev_state));
    assert_eq!(state.registers[1], 2);
    assert_eq!(state.registers[2], 2);
    assert_eq!(state.pc, 44);

    let mut prev_state2 = State::blank_state();
    prev_state2.registers[1] = 2;
    let state2 = compile_and_execute(String::from("BEQ $1 $2 10"), Some(prev_state2));
    assert_eq!(state2.registers[1], 2);
    assert_eq!(state2.registers[2], 0);
    assert_eq!(state2.pc, 4);
}

#[test]
fn bne() {
    let mut prev_state = State::blank_state();
    prev_state.registers[1] = 2;
    prev_state.registers[2] = 2;
    let state = compile_and_execute(String::from("BNE $1 $2 10"), Some(prev_state));
    assert_eq!(state.registers[1], 2);
    assert_eq!(state.registers[2], 2);
    assert_eq!(state.pc, 4);

    let mut prev_state2 = State::blank_state();
    prev_state2.registers[1] = 2;
    let state2 = compile_and_execute(String::from("BNE $1 $2 10"), Some(prev_state2));
    assert_eq!(state2.registers[1], 2);
    assert_eq!(state2.registers[2], 0);
    assert_eq!(state2.pc, 44);
}

#[test]
fn lw() {
    let mut prev_state = State::blank_state();
    prev_state.registers[1] = 1;
    prev_state.memory[1] = 100;
    prev_state.memory[22] = 220;
    let state = compile_and_execute(String::from("LW $2 0($1)\nLW $3 21($1)"), Some(prev_state));
    assert_eq!(state.registers[1], 1);
    assert_eq!(state.registers[2], 100);
    assert_eq!(state.registers[3], 220);
    assert_eq!(state.memory[1], 100);
    assert_eq!(state.memory[22], 220);
}

#[test]
fn sw() {
    let mut prev_state = State::blank_state();
    prev_state.registers[1] = 1;
    prev_state.registers[2] = 22;
    prev_state.registers[3] = 333;
    let state = compile_and_execute(
        String::from("SW $1 0($0)\nSW $2 1($0)\nSW $3 2($0)"),
        Some(prev_state),
    );
    assert_eq!(state.registers[1], 1);
    assert_eq!(state.registers[2], 22);
    assert_eq!(state.registers[3], 333);
    assert_eq!(state.memory[0], 1);
    assert_eq!(state.memory[1], 22);
    assert_eq!(state.memory[2], 333);
}
