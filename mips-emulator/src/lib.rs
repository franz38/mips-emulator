use wasm_bindgen::prelude::*;

pub mod compiler;
pub mod emulator;
pub mod state;

use compiler::compile_code;
use state::{ StateManager };


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn init() -> *mut i32 {
    let mut buf = StateManager::init();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr;
}

#[wasm_bindgen]
pub fn compile(ptr: *mut i32, code: String) {
    let code_lines: Vec<String> = code.split("\n").map(|v| v.to_string()).collect();
    let binary_code: [i32; 30] = compile_code(code_lines);
    let _code_size = code.split("\n").count();

    StateManager::compile(ptr, binary_code);
}

#[wasm_bindgen]
pub fn run(ptr: *mut i32, steps: Option<u32>) {
    let mut v = StateManager::get(ptr);
    v = emulator::run(v, steps);
    std::mem::forget(v);
}

#[wasm_bindgen]
pub fn get_state(ptr: *mut i32) -> Vec<i32> {
    let v = StateManager::get(ptr);
    let cp = v.clone();
    std::mem::forget(v);
    cp
}





