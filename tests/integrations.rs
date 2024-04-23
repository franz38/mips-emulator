use mips_emulator::{compile_and_execute, state::State};

#[test]
fn pc_value() {
    let state = compile_and_execute(String::from("ADDI $5 $0 5"), None, None);
    assert_eq!(state.pc, 4);
    let state2 = compile_and_execute(
        String::from("ADD $1 $0 $2\nADD $1 $0 $2\nSUB $1 $0 $2"),
        None,
        None
    );
    assert_eq!(state2.pc, 12);
}

#[test]
fn prev_state_inferred() {
    let mut prev_state = State::blank_state();
    prev_state.registers[1] = 1;
    prev_state.registers[2] = 2;
    prev_state.registers[3] = 3;
    prev_state.registers[4] = 4;
    let state = compile_and_execute(String::from("ADDI $5 $0 5"), Some(prev_state), None);
    assert_eq!(state.registers[0], 0);
    assert_eq!(state.registers[1], 1);
    assert_eq!(state.registers[2], 2);
    assert_eq!(state.registers[3], 3);
    assert_eq!(state.registers[4], 4);
    assert_eq!(state.registers[5], 5);
    assert_eq!(state.registers[6], 0);
}

#[test]
fn add_test2() {
    let state = compile_and_execute(
        String::from("ADDI $1 $0 22\nADDI $2 $0 33\nADD $3 $1 $2\nADD $4 $3 $1"),
        None,
        None
    );
    assert_eq!(state.registers[0], 0);
    assert_eq!(state.registers[1], 22);
    assert_eq!(state.registers[2], 33);
    assert_eq!(state.registers[3], 55);
    assert_eq!(state.registers[4], 77);
}
