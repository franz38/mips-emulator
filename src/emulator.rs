use crate::state::State;

mod run_i;
mod run_r;
mod run_j;
mod slice_instruction;


pub fn run(binary_code: &[u32], binary_size: usize, prev_state: Option<State>) -> State {
    
    let mut registers: [u32; 32] = [0; 32];
    let mut lo: u32 = 0;
    let mut hi: u32 = 0;
    let mut ir: u32 = 0;
    let mut virtual_pc = 0;
    let mut memory: [u32; 200] = [0; 200];

    match prev_state {
        Some(ps) => {
            registers = ps.registers;
            memory = ps.memory;
            lo = ps.lo;
            hi = ps.hi;
            ir = ps.ir;
            virtual_pc = ps.pc.clone() as usize;
        },
        None => {}
    }

    let mut exit: bool = false;

    while virtual_pc < binary_size && !exit{
        
        // ir load
        ir = binary_code[virtual_pc];
        virtual_pc += 1;

        if slice_instruction::get_op(ir) == 0 {
            exit = run_r::run_r(ir, &mut registers, &mut lo, &mut hi);
        }
        else if slice_instruction::get_op(ir) == 2 || slice_instruction::get_op(ir) == 3 {
            exit = run_j::run_j(ir, &mut registers, &mut virtual_pc);
        }
        else {
            exit = run_i::run_i(ir, &mut registers, &mut virtual_pc, &mut memory);
        }

    }

    let state = State {
        registers,
        memory,
        lo,
        hi,
        ir,
        pc: (virtual_pc*4) as u32
    };

    return state;

}


#[cfg(test)]
mod test_instruction_emulation{
    use crate::state::State;
    use super::run;

    #[test]
    fn add(){
        let mut state = State::blank_state();
        state.registers[8] = 44;
        let binary: [u32; 1] = [0x01092020]; // add $a0, $t0, $t1
        state = run(&binary, 1, Option::Some(state));
        assert_eq!(state.registers[8], 44);
        assert_eq!(state.registers[4], 44);
    }

    #[test]
    fn sub(){
        let mut state = State::blank_state();
        state.registers[15] = 44;
        state.registers[9] = 11;
        let binary: [u32; 1] = [0x01e93022]; // sub $a2, $t7, $t1
        state = run(&binary, 1, Option::Some(state));
        assert_eq!(state.registers[15], 44);
        assert_eq!(state.registers[9], 11);
        assert_eq!(state.registers[6], 33);
    }
}





