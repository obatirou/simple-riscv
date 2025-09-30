Unprivileged RISCV

## Principles:

- Limited set of instructions
- Load-store: only load and store access memory, everything else works on CPU registers. Simplify the design of hardware.
- Fixed length instruction encoding: 32bit long. All registers are in constant locations.
- 32 general purpose integer registers encouraging to keep frequently used variables in them minimizing memory accesses.
- Simplicity makes it convenient for pipelining: processing can process in parallel several instructions.

## Modular ISA:

- a small mandatory base ISA + optional extensions
  RV32I or RV64I: around 40 instructions that can run a complete software stack. 32/64 is the width of registers in bits.
- Extensions:
  - M: Integer Multiplication and Division (mul, div).
  - A: Atomic Instructions (for multi-core synchronization, like amoswap).
  - F: Single-Precision Floating-Point.
  - etc

## Architectural components

### Registers

- 32 general purpose registers: 0x to 0x31
  0x always return 0. No writting to it
- PC: program counter

### ABI names

- sp (0x2): stack pointer
- ra (0x1): return address for function calls
- a0-a7: function arguments and return values
- t0-t6: temporary registers for scratch calculation.
- s0-s11: saved registers long lived accross functino calls.

### Memory model

- byte-addressable: flat array of bytes with unique address
- load store architecture
- typically little endian

### Instruction encoding

- Hierarchical design: opcodes, funct3, funct7
- 6 encoding format depending on instruction: source and destination register fields are kept in the same place across almost all formats

  - R: register to register operations

    `| funct7 | rs2 | rs1 | funct3 | rd | opcode |`

    `| 7 bits | 5b | 5b | 3 bits | 5b | 7 bits |`
  - I: immediate operations

    `| imm[11:0] | rs1 | funct3 | rd | opcode |`

    `| 12 bits | 5b | 3 bits | 5b | 7 bits |`
  - S: store operations

    `| imm[11:5] | rs2 | rs1 | funct3 | imm[4:0] | opcode |`

    `| 7 bits | 5b | 5b | 3 bits | 5 bits | 7 bits |`
  - B: branch operations

    `| imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode |`

    `| 7 bits | 5b | 5b | 3 bits | 5 bits | 7 bits |`
  - U: upper immediate operations

    `| imm[31:12] | rd | opcode |`

    `| 20 bits | 5b | 7 bits |`
  - J: jump operations

    `| imm[20|10:1|11|19:12] | rd | opcode |`

    `| 20 bits | 5 bits | 7 bits |`

- 5 stage pipiline
  - IF: instruction fetch
  - ID: instruction decode
  - EX: execute
  - MEM: memory access
  - WB: write back

See [base instruction set RV32I](./src/static/RV32I.png)

### ZkVMs

1. Write code
2. Compile to unprivileged RISCV RV32I (with extensions or not) ELF
   - Compiler compile to RISC-V assembly
   - Assembler turns assembly into binary machine code
   - Linker takes those files and produces a ELF file.
     - header: entry point address, where to start the program
     - Program header: program memory image, what to load in memory, where is the code, where is the data
     - Section header: helps locate all of the sections of the file
3. Execute instructions and generate trace
4. Generate proof
