// instruction.rs
// Contains information about CPU instructions

use phf::phf_map;

#[derive(PartialEq, Eq)]
pub enum Mnemonic {
    ADC, AND, ASL, BIT, BPL, BMI, BVC, BVS, BCC, BCS, BNE, BEQ, BRK, CMP, CPX, CPY, 
    DEC, EOR, CLC, SEC, CLI, SEI, CLV, CLD, SED, INC, JMP, JSR, LDA, LDX, LDY, LSR,
    NOP, ORA, TAX, TXA, DEX, INX, TAY, TYA, DEY, INY, ROL, ROR, RTI, RTS, SBC, STA,
    TXS, TSX, PHA, PLA, PHP, PLP, STX, STY,
}

#[derive(PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    Zero,
    ZeroX,
    ZeroY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implied,
    Relative,
    Accumulator,
}

struct Instruction {
    opcode: u8,
    mnemonic: Mnemonic,
    mode: AddressingMode,
}

static INSTRUCTIONS: phf::Map<u8, Instruction> = phf_map! {
    // ADC
    0x69 => Instruction{
        opcode: 0x69,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::Immediate,
    },
    0x65 => Instruction{
        opcode: 0x65,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::Zero,
    },
    0x75 => Instruction{
        opcode: 0x75,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::ZeroX,
    },
    0x6d => Instruction{
        opcode: 0x6d,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::Absolute,
    },
    0x7d => Instruction{
        opcode: 0x7d,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::AbsoluteX,
    },
    0x79 => Instruction{
        opcode: 0x79,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::AbsoluteY,
    },
    0x61 => Instruction{
        opcode: 0x61,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::IndirectX,
    },
    0x71 => Instruction{
        opcode: 0x71,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::IndirectY,
    },

    // AND
    0x29 => Instruction{
        opcode: 0x29,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::Immediate,
    },
    0x25 => Instruction{
        opcode: 0x25,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::Zero,
    },
    0x35 => Instruction{
        opcode: 0x35,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::ZeroX,
    },
    0x2d => Instruction{
        opcode: 0x2d,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::Absolute,
    },
    0x3d => Instruction{
        opcode: 0x3d,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::AbsoluteX,
    },
    0x39 => Instruction{
        opcode: 0x39,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::AbsoluteY,
    },
    0x21 => Instruction{
        opcode: 0x21,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::IndirectX,
    },
    0x31 => Instruction{
        opcode: 0x31,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::IndirectY,
    },

    // ASL
    0x0a => Instruction {
        opcode: 0x0a,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::Accumulator,
    },
    0x06 => Instruction{
        opcode: 0x06,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::Zero,
    },
    0x16 => Instruction{
        opcode: 0x16,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::ZeroX,
    },
    0x0e => Instruction{
        opcode: 0x0e,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::Absolute,
    },
    0x1e => Instruction{
        opcode: 0x1e,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::AbsoluteX,
    },

    // BIT
    0x24 => Instruction{
        opcode: 0x24,
        mnemonic: Mnemonic::BIT,
        mode: AddressingMode::Zero,
    },
    0x2c => Instruction{
        opcode: 0x2c,
        mnemonic: Mnemonic::BIT,
        mode: AddressingMode::Absolute,
    },

    // Branching instructions
    0x10 => Instruction{
        opcode: 0x10,
        mnemonic: Mnemonic::BPL,
        mode: AddressingMode::Relative,
    },
    0x30 => Instruction{
        opcode: 0x30,
        mnemonic: Mnemonic::BMI,
        mode: AddressingMode::Relative,
    },
    0x50 => Instruction{
        opcode: 0x50,
        mnemonic: Mnemonic::BVC,
        mode: AddressingMode::Relative,
    },
    0x70 => Instruction{
        opcode: 0x70,
        mnemonic: Mnemonic::BVS,
        mode: AddressingMode::Relative,
    },
    0x90 => Instruction{
        opcode: 0x90,
        mnemonic: Mnemonic::BCC,
        mode: AddressingMode::Relative,
    },
    0xb0 => Instruction{
        opcode: 0xb0,
        mnemonic: Mnemonic::BCS,
        mode: AddressingMode::Relative,
    },
    0xd0 => Instruction{
        opcode: 0xd0,
        mnemonic: Mnemonic::BNE,
        mode: AddressingMode::Relative,
    },
    0xf0 => Instruction{
        opcode: 0xf0,
        mnemonic: Mnemonic::BEQ,
        mode: AddressingMode::Relative,
    },

    // BRK
    0x00 => Instruction{
        opcode: 0x00,
        mnemonic: Mnemonic::BRK,
        mode: AddressingMode::Implied,
    },
    
    // CMP
    0xc9 => Instruction{
        opcode: 0xc9,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::Immediate,
    },
    0xc5 => Instruction{
        opcode: 0xc5,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::Zero,
    },
    0xd5 => Instruction{
        opcode: 0xd5,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::ZeroX,
    },
    0xcd => Instruction{
        opcode: 0xcd,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::Absolute,
    },
    0xdd => Instruction{
        opcode: 0xdd,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::AbsoluteX,
    },
    0xd9 => Instruction{
        opcode: 0xd9,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::AbsoluteY,
    },
    0xc1 => Instruction{
        opcode: 0xc1,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::IndirectX,
    },
    0xd1 => Instruction{
        opcode: 0xd1,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::IndirectY,
    },

    // CPX
    0xe0 => Instruction{
        opcode: 0xe0,
        mnemonic: Mnemonic::CPX,
        mode: AddressingMode::Immediate,
    },
    0xe4 => Instruction{
        opcode: 0xe4,
        mnemonic: Mnemonic::CPX,
        mode: AddressingMode::Zero,
    },
    0xec => Instruction{
        opcode: 0xec,
        mnemonic: Mnemonic::CPX,
        mode: AddressingMode::Absolute,
    },

    // CPY
    0xc0 => Instruction{
        opcode: 0xc0,
        mnemonic: Mnemonic::CPY,
        mode: AddressingMode::Immediate,
    },
    0xc4 => Instruction{
        opcode: 0xc4,
        mnemonic: Mnemonic::CPY,
        mode: AddressingMode::Zero,
    },
    0xcc => Instruction{
        opcode: 0xcc,
        mnemonic: Mnemonic::CPY,
        mode: AddressingMode::Absolute,
    },
};
