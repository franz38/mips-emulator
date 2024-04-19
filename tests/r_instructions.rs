use mips_emulator::{compile_and_execute, state::State};

#[test]
fn add() {
    let mut initial_state = State::blank_state();
    initial_state.registers[1] = 22;
    let state = compile_and_execute(String::from("ADD $2 $0 $1"), Some(initial_state));
    assert_eq!(state.registers[0], 0);
    assert_eq!(state.registers[1], 22);
    assert_eq!(state.registers[2], 22);

    let mut initial_state2 = State::blank_state();
    initial_state2.registers[1] = 22;
    let state2 = compile_and_execute(
        String::from("ADD $2 $0 $1\nADD $3 $2 $2\nADD $4 $3 $2"),
        Some(initial_state2),
    );
    assert_eq!(state2.registers[0], 0);
    assert_eq!(state2.registers[1], 22);
    assert_eq!(state2.registers[2], 22);
    assert_eq!(state2.registers[3], 44);
    assert_eq!(state2.registers[4], 66);
}

#[test]
fn sub() {
    let mut initial_state = State::blank_state();
    initial_state.registers[1] = 22;
    initial_state.registers[2] = 5;
    let state = compile_and_execute(String::from("SUB $3 $1 $2"), Some(initial_state));
    assert_eq!(state.registers[0], 0);
    assert_eq!(state.registers[1], 22);
    assert_eq!(state.registers[2], 5);
    assert_eq!(state.registers[3], 17);

    let mut initial_state2 = State::blank_state();
    initial_state2.registers[1] = 22;
    initial_state2.registers[2] = 5;
    let state2 = compile_and_execute(
        String::from("SUB $3 $1 $2\nSUB $4 $3 $2\nSUB $5 $1 $4"),
        Some(initial_state2),
    );
    assert_eq!(state2.registers[0], 0);
    assert_eq!(state2.registers[1], 22);
    assert_eq!(state2.registers[2], 5);
    assert_eq!(state2.registers[3], 17);
    assert_eq!(state2.registers[4], 12);
    assert_eq!(state2.registers[5], 10);
}

#[test]
fn sll() {
    let mut prev_state = State::blank_state();
    prev_state.registers[1] = 1;
    let state = compile_and_execute(String::from("SLL $1 $1 2"), Some(prev_state));
    assert_eq!(state.registers[1], 4);

    let mut prev_state2 = State::blank_state();
    prev_state2.registers[1] = 1;
    let state2 = compile_and_execute(String::from("SLL $2 $1 3\nSLL $3 $1 5"), Some(prev_state2));
    assert_eq!(state2.registers[1], 1);
    assert_eq!(state2.registers[2], 8);
    assert_eq!(state2.registers[3], 32);
}

#[test]
fn srl() {
    let mut prev_state = State::blank_state();
    prev_state.registers[1] = 16;
    let state = compile_and_execute(String::from("SRL $1 $1 1"), Some(prev_state));
    assert_eq!(state.registers[1], 8);

    let mut prev_state2 = State::blank_state();
    prev_state2.registers[1] = 16;
    let state2 = compile_and_execute(String::from("SRL $2 $1 1\nSRL $3 $1 4"), Some(prev_state2));
    assert_eq!(state2.registers[1], 16);
    assert_eq!(state2.registers[2], 8);
    assert_eq!(state2.registers[3], 1);
}

#[test]
fn mult() {
    let mut prev_state = State::blank_state();
    prev_state.registers[1] = 2;
    prev_state.registers[2] = 100;
    let state = compile_and_execute(String::from("MULT $1 $2"), Some(prev_state));
    assert_eq!(state.registers[1], 2);
    assert_eq!(state.registers[2], 100);
    assert_eq!(state.lo, 200);
    assert_eq!(state.hi, 0);

    let mut prev_state2 = State::blank_state();
    prev_state2.registers[1] = 32;
    prev_state2.registers[2] = 0xFA29FFFE;
    let state2 = compile_and_execute(String::from("MULT $1 $2"), Some(prev_state2));
    assert_eq!(state2.registers[1], 32);
    assert_eq!(state2.registers[2], 0xFA29FFFE);
    assert_eq!(state2.lo, 0x453FFFC0);
    assert_eq!(state2.hi, 0x1F);
}

#[test]
fn mflo() {
    let mut prev_state = State::blank_state();
    prev_state.lo = 77;
    let state = compile_and_execute(String::from("MFLO $1"), Some(prev_state));
    assert_eq!(state.lo, 77);
    assert_eq!(state.registers[1], 77);
}

#[test]
fn mfhi() {
    let mut prev_state = State::blank_state();
    prev_state.hi = 22;
    let state = compile_and_execute(String::from("MFHI $1"), Some(prev_state));
    assert_eq!(state.hi, 22);
    assert_eq!(state.registers[1], 22);
}

#[test]
fn div() {}
