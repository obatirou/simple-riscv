/// RisVInterpreterRV32I

const MASK_7_BITS: u32 = 0b1111111;
const MASK_20_BITS: u32 = 0b11111111111111111111;
const MASK_5_BITS: u32 = 0b11111;
enum Opcode {
    Lui = 0b0110111,
    Auipc = 0b0010111,
    Illegal,
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match value {
            0b0110111 => Opcode::Lui,
            0b0010111 => Opcode::Auipc,
            _ => Opcode::Illegal,
        }
    }
}

pub struct RawInstruction {
    pub bits: u32,
}

impl RawInstruction {
    pub fn rd(&self) -> usize {
        ((self.bits >> 7) & MASK_5_BITS) as usize
    }
    pub fn imm_20(&self) -> u32 {
        self.bits >> 12 & MASK_20_BITS
    }
}

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

    pub fn fetch_instruction(&self) -> RawInstruction {
        let start = self.pc as usize;
        RawInstruction {
            bits: u32::from_be_bytes(self.memory[start..start + 4].try_into().unwrap()),
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.memory.len() as u32 {
            let raw_instruction = self.fetch_instruction();
            self.execute(raw_instruction);
        }
    }

    pub fn execute(&mut self, raw_instruction: RawInstruction) {
        let opcode = Opcode::from(raw_instruction.bits & MASK_7_BITS);
        match opcode {
            Opcode::Lui => self.execute_lui(raw_instruction),
            Opcode::Auipc => self.execute_auipc(raw_instruction),
            _ => panic!("Invalid opcode"),
        }
    }

    // LUI: Load Upper Immediate
    // Loads the upper 20 bits of the immediate value into the destination register.
    pub fn execute_lui(&mut self, raw_instruction: RawInstruction) {
        let rd = raw_instruction.rd();
        let imm_20 = raw_instruction.imm_20();
        self.registers[rd] = imm_20 << 12;
        self.pc += 4;
    }

    // AUIPC: Add Upper Immediate to PC
    // Adds the upper 20 bits of the immediate value to the program counter into the destination register.
    pub fn execute_auipc(&mut self, raw_instruction: RawInstruction) {
        let rd = raw_instruction.rd();
        let imm_20 = raw_instruction.imm_20();
        self.registers[rd] = imm_20 << 12 + self.pc;
        self.pc += 4;
    }
}

fn main() {
    let mut interpreter = Interpreter::new();
    interpreter.load_program(&[0x01, 0x00, 0x00, 0xB7, 0x01, 0x00, 0x00, 0x97]); // lui x1, 0x1000 auipc x1, 0x1000
    interpreter.run();
}
