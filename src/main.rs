/// RisVInterpreterRV32I

const MASK_OPCODE: u32 = 0b1111111;
const MASK_IMM: u32 = 0b11111111111111111111;
enum Opcode {
    Lui = 0b0110111,
    Illegal,
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match value {
            0b0110111 => Opcode::Lui,
            _ => Opcode::Illegal,
        }
    }
}

pub struct RawInstruction {
    pub bits: u32,
}

impl RawInstruction {
    pub fn rd(&self) -> usize {
        ((self.bits >> 7) & 0b11111) as usize
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
        let opcode = Opcode::from(raw_instruction.bits & MASK_OPCODE);
        match opcode {
            Opcode::Lui => self.execute_lui(raw_instruction),
            _ => panic!("Invalid opcode"),
        }
    }

    // LUI: Load Upper Immediate
    // Loads the upper 20 bits of the immediate value into the destination register.
    pub fn execute_lui(&mut self, raw_instruction: RawInstruction) {
        let rd = raw_instruction.rd();
        let imm_20 = raw_instruction.bits >> 12 & MASK_IMM;
        self.registers[rd] = imm_20 << 12;
        self.pc += 4;
    }
}

fn main() {
    let mut interpreter = Interpreter::new();
    interpreter.load_program(&[0x01, 0x00, 0x00, 0xB7]); // lui x1, 0x1000
    interpreter.run();
}
