
pub struct State{
    pub registers: [u32; 32],
    pub memory: [u32; 200],
    pub lo: u32,
    pub hi: u32,
    pub pc: u32,
    pub ir: u32
}

impl State {
    pub fn blank_state() -> Self{
        return Self {
            registers: [0; 32],
            memory: [0; 200],
            lo: 0,
            hi: 0,
            ir: 0,
            pc: 0
        };
    }
}