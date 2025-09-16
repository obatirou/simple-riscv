/// RisVInterpreterRV32I

const LOAD: u32 = 0b0000011;
const MEM: u32 = 0b0001111;
const OP_IMM: u32 = 0b0010011;
const AUIPC: u32 = 0b0010111;
const STORE: u32 = 0b0100011;
const OP: u32 = 0b0110011;
const LUI: u32 = 0b0110111;
const BRANCH: u32 = 0b1100011;
const JALR: u32 = 0b1100111;
const JAL: u32 = 0b1101111;
const SYSTEM: u32 = 0b1110011;

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
        let raw_instruction = self.memory[start..start + 4].to_vec();
        u32::from_le_bytes(raw_instruction.try_into().unwrap())
    }

    pub fn decode_instruction(&self, instruction: u32) -> Instruction {
        let opcode = instruction & 0b1111111;
    }
}
