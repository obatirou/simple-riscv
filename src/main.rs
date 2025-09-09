/// RisVInterpreterRV32I

struct Interpreter {
    memory: Vec<u8>,
    pc: u32,
    registers: [u32; 32],
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: Vec::new(),
            pc: 0,
            registers: [0; 32],
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.extend_from_slice(program);
    }


    pub fn fetch_instruction(&self) -> u32 {
        let start = self.pc as usize;
        let instruction = self.memory[start..start + 4].to_vec();
        u32::from_le_bytes(instruction.try_into().unwrap())
    }

    // pub fn decode_instruction(&self, instruction: u32) -> Instruction {}
}
