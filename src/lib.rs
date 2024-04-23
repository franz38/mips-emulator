use wasm_bindgen::prelude::*;

pub mod compiler;
pub mod emulator;
pub mod state;

use compiler::compile_code;
use emulator::run;
use state::{Abc, State};


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compile_and_execute_js(content: String, prev_state: Option<Vec<u32>>, steps: Option<u32>) -> Vec<u32> {
    let prev_state_parsed = match prev_state {
        Some(vec) => State::from_vec(&vec),
        None => State::blank_state(),
    };
    let state = compile_and_execute(content, Option::Some(prev_state_parsed), steps);
    return state.to_vec();
}


pub fn compile_and_execute(content: String, prev_state: Option<State>, steps: Option<u32>) -> State {
    let code_lines: Vec<String> = content.split("\n").map(|v| v.to_string()).collect();
    let binary_code: [u32; 30] = compile_code(code_lines);
    let code_size = content.split("\n").count();

    let step_amount: u32 = match steps {
        Some(s) => s,
        None => u32::max_value()
    };
    
    run(&binary_code, code_size, prev_state, step_amount)
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
pub fn arrTest() -> Vec<u32> {
    let mut k = Vec::<u32>::new();
    k.push(1);
    k.push(2);
    return k;
}

// #[wasm_bindgen]
// pub fn vecTest() -> [u32] {
//     let mut k: [u32; 32] = [0, 32];
//     return k;
// }

// #[wasm_bindgen]
// pub fn getAbc() -> Abc {
//     let m = Abc {
//         abc: 33
//     };
//     return m;
// }
