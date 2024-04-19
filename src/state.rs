use crate::compiler::utils::get_register_alias;


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