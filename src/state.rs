use crate::compiler::utils::get_register_alias;

// #[wasm_bindgen]
pub struct State{
    pub registers: [u32; 32],
    pub memory: [u32; 200],
    pub lo: u32,
    pub hi: u32,
    pub pc: u32,
    pub ir: u32
}

// pub struct  State2 {

// }


pub struct Abc {
    pub abc: u32,
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

    pub fn from_vec(vec: &Vec<u32>) -> Self {
        let mut s = Self {
            registers: [0; 32],
            memory: [0; 200],
            lo: vec[2],
            hi: vec[3],
            ir: vec[1],
            pc: vec[0]
        };
        
        for i in 0..32 {
            s.registers[i] = vec[i+4];
        }

        for i in 0..200 {
            s.memory[i] = vec[i+36];
        }

        return  s;
    }

    pub fn to_vec(&self) -> Vec<u32> {
        let mut v = Vec::<u32>::new();
        v.push(self.pc);
        v.push(self.ir);
        v.push(self.lo);
        v.push(self.hi);

        for i in 0..32 {
            v.push(self.registers[i]);
        }

        for i in 0..200 {
            v.push(self.memory[i]);
        }

        return v;
    }

    fn var_to_string(label1: &str, value1: &u32, label2: &str, value2: &u32) -> String{
        return format!("{}\n| {:2} | {:10} | {:2} | {:10} |", Self::end_line(), label1, value1, label2, value2);
    }

    fn end_line() -> String {
        let mut l: [char; 37] = ['-'; 37];
        l[0] = '+'; l[36] = '+';
        return l.iter().collect::<String>();
    }
    

    pub fn to_string(&self) -> String{

        let registers_table = (0..16)
            .map(|i| Self::var_to_string(
                get_register_alias(&i.to_string()), 
                &self.registers[i], 
                get_register_alias(&(i+16).to_string()), 
                &self.registers[i+16]) + "\n")
            .collect::<String>();

        return String::from(
            Self::var_to_string("PC", &self.pc, "IR", &self.ir) + "\n" +
            &Self::var_to_string("LO", &self.lo, "HI", &self.hi) + "\n" + 
            &Self::end_line() + "\n\n" +
            &registers_table +
            &Self::end_line()
        );
    }
}

#[cfg(test)]
mod state_conversion_test{
    use super::State;


    #[test]
    fn state_2_vec(){
        let mut state = State::blank_state();
        state.registers[0] = 44;
        state.registers[10] = 77;
        state.hi = 33;

        let mut v_state: Vec<u32> = vec![0; 36];
        v_state[3] = 33;
        v_state[4] = 44;
        v_state[14] = 77;

        assert_eq!(v_state, state.to_vec());
        assert_eq!(State::from_vec(&v_state).registers[10], 77);
        assert_eq!(State::from_vec(&v_state).registers[0], 44);
        assert_eq!(State::from_vec(&v_state).hi, 33);
    }
}
