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
pub fn init() -> *mut u32 {
    let mut buf = StateManager::init();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr;
}

#[wasm_bindgen]
pub fn compile(ptr: *mut u32, code: String) {
    let code_lines: Vec<String> = code.split("\n").map(|v| v.to_string()).collect();
    let binary_code: [u32; 30] = compile_code(code_lines);
    let code_size = code.split("\n").count();

    StateManager::store_code(ptr, binary_code);
}

#[wasm_bindgen]
pub fn run(ptr: *mut u32, steps: Option<u32>) {
    let mut v = StateManager::get(ptr);
    v = emulator::run(v, steps);
    std::mem::forget(v);
}

#[wasm_bindgen]
pub fn get_state(ptr: *mut u32) -> Vec<u32> {
    let v = StateManager::get(ptr);
    let cp = v.clone();
    std::mem::forget(v);
    cp
}





