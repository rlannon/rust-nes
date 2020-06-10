// instruction.rs
// Contains information about CPU instructions

use phf::phf_map;

#[derive(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
pub enum Mnemonic {
    ADC, AND, ASL, BIT, BPL, BMI, BVC, BVS, BCC, BCS, BNE, BEQ, BRK, CMP, CPX, CPY, 
    DEC, EOR, CLC, SEC, CLI, SEI, CLV, CLD, SED, INC, JMP, JSR, LDA, LDX, LDY, LSR,
    NOP, ORA, TAX, TXA, DEX, INX, TAY, TYA, DEY, INY, ROL, ROR, RTI, RTS, SBC, STA,
    TXS, TSX, PHA, PLA, PHP, PLP, STX, STY,
}

#[derive(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    opcode: u8,
    pub mnemonic: Mnemonic,
    pub mode: AddressingMode,
}

pub static INSTRUCTIONS: phf::Map<u8, Instruction> = phf_map! {
    // ADC
    0x69u8 => Instruction{
        opcode: 0x69,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::Immediate,
    },
    0x65u8 => Instruction{
        opcode: 0x65,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::Zero,
    },
    0x75u8 => Instruction{
        opcode: 0x75,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::ZeroX,
    },
    0x6du8 => Instruction{
        opcode: 0x6d,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::Absolute,
    },
    0x7du8 => Instruction{
        opcode: 0x7d,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::AbsoluteX,
    },
    0x79u8 => Instruction{
        opcode: 0x79,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::AbsoluteY,
    },
    0x61u8 => Instruction{
        opcode: 0x61,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::IndirectX,
    },
    0x71u8 => Instruction{
        opcode: 0x71,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::IndirectY,
    },

    // AND
    0x29u8 => Instruction{
        opcode: 0x29,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::Immediate,
    },
    0x25u8 => Instruction{
        opcode: 0x25,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::Zero,
    },
    0x35u8 => Instruction{
        opcode: 0x35,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::ZeroX,
    },
    0x2du8 => Instruction{
        opcode: 0x2d,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::Absolute,
    },
    0x3du8 => Instruction{
        opcode: 0x3d,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::AbsoluteX,
    },
    0x39u8 => Instruction{
        opcode: 0x39,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::AbsoluteY,
    },
    0x21u8 => Instruction{
        opcode: 0x21,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::IndirectX,
    },
    0x31u8 => Instruction{
        opcode: 0x31,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::IndirectY,
    },

    // ASL
    0x0au8 => Instruction {
        opcode: 0x0a,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::Accumulator,
    },
    0x06u8 => Instruction{
        opcode: 0x06,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::Zero,
    },
    0x16u8 => Instruction{
        opcode: 0x16,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::ZeroX,
    },
    0x0eu8 => Instruction{
        opcode: 0x0e,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::Absolute,
    },
    0x1eu8 => Instruction{
        opcode: 0x1e,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::AbsoluteX,
    },

    // BIT
    0x24u8 => Instruction{
        opcode: 0x24,
        mnemonic: Mnemonic::BIT,
        mode: AddressingMode::Zero,
    },
    0x2cu8 => Instruction{
        opcode: 0x2c,
        mnemonic: Mnemonic::BIT,
        mode: AddressingMode::Absolute,
    },

    // Branching instructions
    0x10u8 => Instruction{
        opcode: 0x10,
        mnemonic: Mnemonic::BPL,
        mode: AddressingMode::Relative,
    },
    0x30u8 => Instruction{
        opcode: 0x30,
        mnemonic: Mnemonic::BMI,
        mode: AddressingMode::Relative,
    },
    0x50u8 => Instruction{
        opcode: 0x50,
        mnemonic: Mnemonic::BVC,
        mode: AddressingMode::Relative,
    },
    0x70u8 => Instruction{
        opcode: 0x70,
        mnemonic: Mnemonic::BVS,
        mode: AddressingMode::Relative,
    },
    0x90u8 => Instruction{
        opcode: 0x90,
        mnemonic: Mnemonic::BCC,
        mode: AddressingMode::Relative,
    },
    0xb0u8 => Instruction{
        opcode: 0xb0,
        mnemonic: Mnemonic::BCS,
        mode: AddressingMode::Relative,
    },
    0xd0u8 => Instruction{
        opcode: 0xd0,
        mnemonic: Mnemonic::BNE,
        mode: AddressingMode::Relative,
    },
    0xf0u8 => Instruction{
        opcode: 0xf0,
        mnemonic: Mnemonic::BEQ,
        mode: AddressingMode::Relative,
    },

    // BRK
    0x00u8 => Instruction{
        opcode: 0x00,
        mnemonic: Mnemonic::BRK,
        mode: AddressingMode::Implied,
    },
    
    // CMP
    0xc9u8 => Instruction{
        opcode: 0xc9,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::Immediate,
    },
    0xc5u8 => Instruction{
        opcode: 0xc5,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::Zero,
    },
    0xd5u8 => Instruction{
        opcode: 0xd5,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::ZeroX,
    },
    0xcdu8 => Instruction{
        opcode: 0xcd,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::Absolute,
    },
    0xddu8 => Instruction{
        opcode: 0xdd,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::AbsoluteX,
    },
    0xd9u8 => Instruction{
        opcode: 0xd9,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::AbsoluteY,
    },
    0xc1u8 => Instruction{
        opcode: 0xc1,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::IndirectX,
    },
    0xd1u8 => Instruction{
        opcode: 0xd1,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::IndirectY,
    },

    // CPX
    0xe0u8 => Instruction{
        opcode: 0xe0,
        mnemonic: Mnemonic::CPX,
        mode: AddressingMode::Immediate,
    },
    0xe4u8 => Instruction{
        opcode: 0xe4,
        mnemonic: Mnemonic::CPX,
        mode: AddressingMode::Zero,
    },
    0xecu8 => Instruction{
        opcode: 0xec,
        mnemonic: Mnemonic::CPX,
        mode: AddressingMode::Absolute,
    },

    // CPY
    0xc0u8 => Instruction{
        opcode: 0xc0,
        mnemonic: Mnemonic::CPY,
        mode: AddressingMode::Immediate,
    },
    0xc4u8 => Instruction{
        opcode: 0xc4,
        mnemonic: Mnemonic::CPY,
        mode: AddressingMode::Zero,
    },
    0xccu8 => Instruction{
        opcode: 0xcc,
        mnemonic: Mnemonic::CPY,
        mode: AddressingMode::Absolute,
    },

    // DEC
    0xc6u8 => Instruction{
        opcode: 0xc6,
        mnemonic: Mnemonic::DEC,
        mode: AddressingMode::Zero,
    },
    0xd6u8 => Instruction {
        opcode: 0xd6,
        mnemonic: Mnemonic::DEC,
        mode: AddressingMode::ZeroX,
    },
    0xceu8 => Instruction{
        opcode: 0xce,
        mnemonic: Mnemonic::DEC,
        mode: AddressingMode::Absolute,
    },
    0xdeu8 => Instruction{
        opcode: 0xde,
        mnemonic: Mnemonic::DEC,
        mode: AddressingMode::AbsoluteX,
    },

    // EOR
    0x49u8 => Instruction{
        opcode: 0x49,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::Immediate,
    },
    0x45u8 => Instruction{
        opcode: 0x45,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::Zero,
    },
    0x55u8 => Instruction{
        opcode: 0x55,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::ZeroX,
    },
    0x4du8 => Instruction{
        opcode: 0x4d,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::Absolute,
    },
    0x5du8 => Instruction{
        opcode: 0x5d,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::AbsoluteX,
    },
    0x59u8 => Instruction{
        opcode: 0x59,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::AbsoluteY,
    },
    0x41u8 => Instruction{
        opcode: 0x41,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::IndirectX,
    },
    0x51u8 => Instruction{
        opcode: 0x51,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::IndirectY,
    },

    // Flag instructions
    0x18u8 => Instruction{
        opcode: 0x18,
        mnemonic: Mnemonic::CLC,
        mode: AddressingMode::Implied,
    },
    0x38u8 => Instruction{
        opcode: 0x38,
        mnemonic: Mnemonic::SEC,
        mode: AddressingMode::Implied,
    },
    0x58u8 => Instruction{
        opcode: 0x58,
        mnemonic: Mnemonic::CLI,
        mode: AddressingMode::Implied,
    },
    0x78u8 => Instruction{
        opcode: 0x78,
        mnemonic: Mnemonic::SEI,
        mode: AddressingMode::Implied,
    },
    0xb8u8 => Instruction{
        opcode: 0xb8,
        mnemonic: Mnemonic::CLV,
        mode: AddressingMode::Implied,
    },
    0xd8u8 => Instruction{
        opcode: 0xd8,
        mnemonic: Mnemonic::CLD,
        mode: AddressingMode::Implied,
    },
    0xf8u8 => Instruction{
        opcode: 0xf8,
        mnemonic: Mnemonic::SED,
        mode: AddressingMode::Implied,
    },

    // INC
    0xe6u8 => Instruction{
        opcode: 0xe6,
        mnemonic: Mnemonic::INC,
        mode: AddressingMode::Zero,
    },
    0xf6u8 => Instruction{
        opcode: 0xf6,
        mnemonic: Mnemonic::INC,
        mode: AddressingMode::ZeroX,
    },
    0xeeu8 => Instruction{
        opcode: 0xee,
        mnemonic: Mnemonic::INC,
        mode: AddressingMode::Absolute,
    },
    0xfeu8 => Instruction{
        opcode: 0xfe,
        mnemonic: Mnemonic::INC,
        mode: AddressingMode::AbsoluteX,
    },

    // JMP
    0x4cu8 => Instruction{
        opcode: 0x4c,
        mnemonic: Mnemonic::JMP,
        mode: AddressingMode::Absolute,
    },
    0x6cu8 => Instruction{
        opcode: 0x6c,
        mnemonic: Mnemonic::JMP,
        mode: AddressingMode::Indirect,
    },

    // JSR
    0x20u8 => Instruction{
        opcode: 0x20,
        mnemonic: Mnemonic::JSR,
        mode: AddressingMode::Absolute,
    },

    // LDA

    // LDX

    // LDY

    // LSR

    // NOP
    0xeau8 => Instruction{
        opcode: 0xea,
        mnemonic: Mnemonic::NOP,
        mode: AddressingMode::Implied,
    },

    // ORA

    // Register transfer instructions

    // ROL

    // ROR

    // RTI

    // RTS

    // SBC

    // STA

    // Stack instructions

    // STX

    // STY
};
