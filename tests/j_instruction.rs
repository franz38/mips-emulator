use mips_emulator::compile_and_execute;

#[test]
fn j() {
    let state = compile_and_execute(String::from("J 2"), None);
    assert_eq!(state.pc, 12);
}

#[test]
fn jal() {
    let state = compile_and_execute(String::from("JAL 10"), None);
    assert_eq!(state.pc, 44);
    assert_eq!(state.registers[31], 4);
}
