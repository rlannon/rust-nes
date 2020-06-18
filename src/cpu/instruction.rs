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
    pub time: u8,
}

// todo: unofficial/illegal opcodes
pub static INSTRUCTIONS: phf::Map<u8, Instruction> = phf_map! {
    // ADC
    0x69u8 => Instruction{
        opcode: 0x69,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0x65u8 => Instruction{
        opcode: 0x65,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0x75u8 => Instruction{
        opcode: 0x75,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0x6du8 => Instruction{
        opcode: 0x6d,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0x7du8 => Instruction{
        opcode: 0x7d,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::AbsoluteX,
        time: 4,
    },
    0x79u8 => Instruction{
        opcode: 0x79,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::AbsoluteY,
        time: 4,
    },
    0x61u8 => Instruction{
        opcode: 0x61,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::IndirectX,
        time: 6,
    },
    0x71u8 => Instruction{
        opcode: 0x71,
        mnemonic: Mnemonic::ADC,
        mode: AddressingMode::IndirectY,
        time: 5,
    },

    // AND
    0x29u8 => Instruction{
        opcode: 0x29,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0x25u8 => Instruction{
        opcode: 0x25,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0x35u8 => Instruction{
        opcode: 0x35,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0x2du8 => Instruction{
        opcode: 0x2d,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0x3du8 => Instruction{
        opcode: 0x3d,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::AbsoluteX,
        time: 4,
    },
    0x39u8 => Instruction{
        opcode: 0x39,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::AbsoluteY,
        time: 4,
    },
    0x21u8 => Instruction{
        opcode: 0x21,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::IndirectX,
        time: 6,
    },
    0x31u8 => Instruction{
        opcode: 0x31,
        mnemonic: Mnemonic::AND,
        mode: AddressingMode::IndirectY,
        time: 5,
    },

    // ASL
    0x0au8 => Instruction {
        opcode: 0x0a,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::Accumulator,
        time: 2,
    },
    0x06u8 => Instruction{
        opcode: 0x06,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::Zero,
        time: 5,
    },
    0x16u8 => Instruction{
        opcode: 0x16,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::ZeroX,
        time: 6,
    },
    0x0eu8 => Instruction{
        opcode: 0x0e,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::Absolute,
        time: 6,
    },
    0x1eu8 => Instruction{
        opcode: 0x1e,
        mnemonic: Mnemonic::ASL,
        mode: AddressingMode::AbsoluteX,
        time: 7,
    },

    // BIT
    0x24u8 => Instruction{
        opcode: 0x24,
        mnemonic: Mnemonic::BIT,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0x2cu8 => Instruction{
        opcode: 0x2c,
        mnemonic: Mnemonic::BIT,
        mode: AddressingMode::Absolute,
        time: 4,
    },

    /*
    
    Branching instructions
    Note that the cycles (time) listed here are inaccurate, technically;
    * a branch  not taken requires 2 cycles
    * a branch taken adds 1 cycle
    * if a page boundary is crossed, another cycle is added
    This averages out to about 3 cycles, which is what we will use here

    */
    0x10u8 => Instruction{
        opcode: 0x10,
        mnemonic: Mnemonic::BPL,
        mode: AddressingMode::Relative,
        time: 3,
    },
    0x30u8 => Instruction{
        opcode: 0x30,
        mnemonic: Mnemonic::BMI,
        mode: AddressingMode::Relative,
        time: 3,
    },
    0x50u8 => Instruction{
        opcode: 0x50,
        mnemonic: Mnemonic::BVC,
        mode: AddressingMode::Relative,
        time: 3,
    },
    0x70u8 => Instruction{
        opcode: 0x70,
        mnemonic: Mnemonic::BVS,
        mode: AddressingMode::Relative,
        time: 3,
    },
    0x90u8 => Instruction{
        opcode: 0x90,
        mnemonic: Mnemonic::BCC,
        mode: AddressingMode::Relative,
        time: 3,
    },
    0xb0u8 => Instruction{
        opcode: 0xb0,
        mnemonic: Mnemonic::BCS,
        mode: AddressingMode::Relative,
        time: 3,
    },
    0xd0u8 => Instruction{
        opcode: 0xd0,
        mnemonic: Mnemonic::BNE,
        mode: AddressingMode::Relative,
        time: 3,
    },
    0xf0u8 => Instruction{
        opcode: 0xf0,
        mnemonic: Mnemonic::BEQ,
        mode: AddressingMode::Relative,
        time: 3,
    },

    // BRK
    0x00u8 => Instruction{
        opcode: 0x00,
        mnemonic: Mnemonic::BRK,
        mode: AddressingMode::Implied,
        time: 7,
    },
    
    // CMP
    0xc9u8 => Instruction{
        opcode: 0xc9,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0xc5u8 => Instruction{
        opcode: 0xc5,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0xd5u8 => Instruction{
        opcode: 0xd5,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0xcdu8 => Instruction{
        opcode: 0xcd,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0xddu8 => Instruction{
        opcode: 0xdd,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::AbsoluteX,
        time: 4,
    },
    0xd9u8 => Instruction{
        opcode: 0xd9,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::AbsoluteY,
        time: 4,
    },
    0xc1u8 => Instruction{
        opcode: 0xc1,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::IndirectX,
        time: 6,
    },
    0xd1u8 => Instruction{
        opcode: 0xd1,
        mnemonic: Mnemonic::CMP,
        mode: AddressingMode::IndirectY,
        time: 5,
    },

    // CPX
    0xe0u8 => Instruction{
        opcode: 0xe0,
        mnemonic: Mnemonic::CPX,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0xe4u8 => Instruction{
        opcode: 0xe4,
        mnemonic: Mnemonic::CPX,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0xecu8 => Instruction{
        opcode: 0xec,
        mnemonic: Mnemonic::CPX,
        mode: AddressingMode::Absolute,
        time: 4,
    },

    // CPY
    0xc0u8 => Instruction{
        opcode: 0xc0,
        mnemonic: Mnemonic::CPY,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0xc4u8 => Instruction{
        opcode: 0xc4,
        mnemonic: Mnemonic::CPY,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0xccu8 => Instruction{
        opcode: 0xcc,
        mnemonic: Mnemonic::CPY,
        mode: AddressingMode::Absolute,
        time: 4,
    },

    // DEC
    0xc6u8 => Instruction{
        opcode: 0xc6,
        mnemonic: Mnemonic::DEC,
        mode: AddressingMode::Zero,
        time: 5,
    },
    0xd6u8 => Instruction {
        opcode: 0xd6,
        mnemonic: Mnemonic::DEC,
        mode: AddressingMode::ZeroX,
        time: 6,
    },
    0xceu8 => Instruction{
        opcode: 0xce,
        mnemonic: Mnemonic::DEC,
        mode: AddressingMode::Absolute,
        time: 6,
    },
    0xdeu8 => Instruction{
        opcode: 0xde,
        mnemonic: Mnemonic::DEC,
        mode: AddressingMode::AbsoluteX,
        time: 7,
    },

    // EOR
    0x49u8 => Instruction{
        opcode: 0x49,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0x45u8 => Instruction{
        opcode: 0x45,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0x55u8 => Instruction{
        opcode: 0x55,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0x4du8 => Instruction{
        opcode: 0x4d,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0x5du8 => Instruction{
        opcode: 0x5d,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::AbsoluteX,
        time: 4,
    },
    0x59u8 => Instruction{
        opcode: 0x59,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::AbsoluteY,
        time: 4,
    },
    0x41u8 => Instruction{
        opcode: 0x41,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::IndirectX,
        time: 6,
    },
    0x51u8 => Instruction{
        opcode: 0x51,
        mnemonic: Mnemonic::EOR,
        mode: AddressingMode::IndirectY,
        time: 5,
    },

    // Flag instructions
    // All of these require two cycles
    0x18u8 => Instruction{
        opcode: 0x18,
        mnemonic: Mnemonic::CLC,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0x38u8 => Instruction{
        opcode: 0x38,
        mnemonic: Mnemonic::SEC,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0x58u8 => Instruction{
        opcode: 0x58,
        mnemonic: Mnemonic::CLI,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0x78u8 => Instruction{
        opcode: 0x78,
        mnemonic: Mnemonic::SEI,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0xb8u8 => Instruction{
        opcode: 0xb8,
        mnemonic: Mnemonic::CLV,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0xd8u8 => Instruction{
        opcode: 0xd8,
        mnemonic: Mnemonic::CLD,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0xf8u8 => Instruction{
        opcode: 0xf8,
        mnemonic: Mnemonic::SED,
        mode: AddressingMode::Implied,
        time: 2,
    },

    // INC
    0xe6u8 => Instruction{
        opcode: 0xe6,
        mnemonic: Mnemonic::INC,
        mode: AddressingMode::Zero,
        time: 5,
    },
    0xf6u8 => Instruction{
        opcode: 0xf6,
        mnemonic: Mnemonic::INC,
        mode: AddressingMode::ZeroX,
        time: 6,
    },
    0xeeu8 => Instruction{
        opcode: 0xee,
        mnemonic: Mnemonic::INC,
        mode: AddressingMode::Absolute,
        time: 6,
    },
    0xfeu8 => Instruction{
        opcode: 0xfe,
        mnemonic: Mnemonic::INC,
        mode: AddressingMode::AbsoluteX,
        time: 7,
    },

    // JMP
    0x4cu8 => Instruction{
        opcode: 0x4c,
        mnemonic: Mnemonic::JMP,
        mode: AddressingMode::Absolute,
        time: 3,
    },
    0x6cu8 => Instruction{
        opcode: 0x6c,
        mnemonic: Mnemonic::JMP,
        mode: AddressingMode::Indirect,
        time: 5,
    },

    // JSR
    0x20u8 => Instruction{
        opcode: 0x20,
        mnemonic: Mnemonic::JSR,
        mode: AddressingMode::Absolute,
        time: 6,
    },

    // LDA
    0xa9u8 => Instruction{
        opcode: 0xa9,
        mnemonic: Mnemonic::LDA,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0xa5u8 => Instruction{
        opcode: 0xa5,
        mnemonic: Mnemonic::LDA,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0xb5u8 => Instruction{
        opcode: 0xb5,
        mnemonic: Mnemonic::LDA,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0xadu8 => Instruction{
        opcode: 0xad,
        mnemonic: Mnemonic::LDA,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0xbdu8 => Instruction{
        opcode: 0xbd,
        mnemonic: Mnemonic::LDA,
        mode: AddressingMode::AbsoluteX,
        time: 4,
    },
    0xb9u8 => Instruction{
        opcode: 0xb9,
        mnemonic: Mnemonic::LDA,
        mode: AddressingMode::AbsoluteY,
        time: 4,
    },
    0xa1u8 => Instruction{
        opcode: 0xa1,
        mnemonic: Mnemonic::LDA,
        mode: AddressingMode::IndirectX,
        time: 6,
    },
    0xb1u8 => Instruction{
        opcode: 0xb1,
        mnemonic: Mnemonic::LDA,
        mode: AddressingMode::IndirectY,
        time: 5,
    },

    // LDX
    0xa2u8 => Instruction{
        opcode: 0xa2,
        mnemonic: Mnemonic::LDX,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0xa6u8 => Instruction{
        opcode: 0xa6,
        mnemonic: Mnemonic::LDX,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0xb6u8 => Instruction{
        opcode: 0xb6,
        mnemonic: Mnemonic::LDX,
        mode: AddressingMode::ZeroY,
        time: 4,
    },
    0xaeu8 => Instruction{
        opcode: 0xae,
        mnemonic: Mnemonic::LDX,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0xbeu8 => Instruction{
        opcode: 0xbe,
        mnemonic: Mnemonic::LDX,
        mode: AddressingMode::AbsoluteY,
        time: 4,
    },

    // LDY
    0xa0u8 => Instruction{
        opcode: 0xa0,
        mnemonic: Mnemonic::LDY,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0xa4u8 => Instruction{
        opcode: 0xa4,
        mnemonic: Mnemonic::LDY,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0xb4u8 => Instruction{
        opcode: 0xb4,
        mnemonic: Mnemonic::LDY,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0xacu8 => Instruction{
        opcode: 0xac,
        mnemonic: Mnemonic::LDY,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0xbcu8 => Instruction{
        opcode: 0xbc,
        mnemonic: Mnemonic::LDY,
        mode: AddressingMode::AbsoluteX,
        time: 4,
    },

    // LSR
    0x4au8 => Instruction{
        opcode: 0x4a,
        mnemonic: Mnemonic::LSR,
        mode: AddressingMode::Accumulator,
        time: 2,
    },
    0x46u8 => Instruction{
        opcode: 0x46,
        mnemonic: Mnemonic::LSR,
        mode: AddressingMode::Zero,
        time: 5,
    },
    0x56u8 => Instruction{
        opcode: 0x56,
        mnemonic: Mnemonic::LSR,
        mode: AddressingMode::ZeroX,
        time: 6,
    },
    0x4eu8 => Instruction{
        opcode: 0x4e,
        mnemonic: Mnemonic::LSR,
        mode: AddressingMode::Absolute,
        time: 6,
    },
    0x5eu8 => Instruction{
        opcode: 0x5e,
        mnemonic: Mnemonic::LSR,
        mode: AddressingMode::AbsoluteX,
        time: 7,
    },

    // NOP
    0xeau8 => Instruction{
        opcode: 0xea,
        mnemonic: Mnemonic::NOP,
        mode: AddressingMode::Implied,
        time: 2,
    },

    // ORA
    0x09u8 => Instruction{
        opcode: 0x09,
        mnemonic: Mnemonic::ORA,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0x05u8 => Instruction{
        opcode: 0x05,
        mnemonic: Mnemonic::ORA,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0x15u8 => Instruction{
        opcode: 0x15,
        mnemonic: Mnemonic::ORA,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0x0du8 => Instruction{
        opcode: 0x0d,
        mnemonic: Mnemonic::ORA,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0x1du8 => Instruction{
        opcode: 0x1d,
        mnemonic: Mnemonic::ORA,
        mode: AddressingMode::AbsoluteX,
        time: 4,
    },
    0x19u8 => Instruction{
        opcode: 0x19,
        mnemonic: Mnemonic::ORA,
        mode: AddressingMode::AbsoluteY,
        time: 4,
    },
    0x01u8 => Instruction{
        opcode: 0x01,
        mnemonic: Mnemonic::ORA,
        mode: AddressingMode::IndirectX,
        time: 6,
    },
    0x11u8 => Instruction{
        opcode: 0x11,
        mnemonic: Mnemonic::ORA,
        mode: AddressingMode::IndirectY,
        time: 5,
    },

    // Register instructions
    // These require two cycles
    0xaau8 => Instruction{
        opcode: 0xaa,
        mnemonic: Mnemonic::TAX,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0x8au8 => Instruction{
        opcode: 0x8a,
        mnemonic: Mnemonic::TXA,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0xcau8 => Instruction{
        opcode: 0xca,
        mnemonic: Mnemonic::DEX,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0xe8u8 => Instruction{
        opcode: 0xe8,
        mnemonic: Mnemonic::INX,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0xa8u8 => Instruction{
        opcode: 0xa8,
        mnemonic: Mnemonic::TAY,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0x98u8 => Instruction{
        opcode: 0x98,
        mnemonic: Mnemonic::TYA,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0x88u8 => Instruction{
        opcode: 0x88,
        mnemonic: Mnemonic::DEY,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0xc8u8 => Instruction{
        opcode: 0xc8,
        mnemonic: Mnemonic::INY,
        mode: AddressingMode::Implied,
        time: 2,
    },

    // ROL
    0x2au8 => Instruction{
        opcode: 0x2a,
        mnemonic: Mnemonic::ROL,
        mode: AddressingMode::Accumulator,
        time: 2,
    },
    0x26u8 => Instruction{
        opcode: 0x26,
        mnemonic: Mnemonic::ROL,
        mode: AddressingMode::Zero,
        time: 5,
    },
    0x36u8 => Instruction{
        opcode: 0x36,
        mnemonic: Mnemonic::ROL,
        mode: AddressingMode::ZeroX,
        time: 6,
    },
    0x2eu8 => Instruction{
        opcode: 0x2e,
        mnemonic: Mnemonic::ROL,
        mode: AddressingMode::Absolute,
        time: 6,
    },
    0x3eu8 => Instruction{
        opcode: 0x3e,
        mnemonic: Mnemonic::ROL,
        mode: AddressingMode::AbsoluteX,
        time: 7,
    },

    // ROR
    0x6au8 => Instruction{
        opcode: 0x6a,
        mnemonic: Mnemonic::ROR,
        mode: AddressingMode::Accumulator,
        time: 2,
    },
    0x66u8 => Instruction{
        opcode: 0x66,
        mnemonic: Mnemonic::ROR,
        mode: AddressingMode::Zero,
        time: 5,
    },
    0x76u8 => Instruction{
        opcode: 0x76,
        mnemonic: Mnemonic::ROR,
        mode: AddressingMode::ZeroX,
        time: 6,
    },
    0x6eu8 => Instruction{
        opcode: 0x6e,
        mnemonic: Mnemonic::ROR,
        mode: AddressingMode::Absolute,
        time: 6,
    },
    0x7eu8 => Instruction{
        opcode: 0x7e,
        mnemonic: Mnemonic::ROR,
        mode: AddressingMode::AbsoluteX,
        time: 7,
    },

    // RTI
    0x40u8 => Instruction{
        opcode: 0x40,
        mnemonic: Mnemonic::RTI,
        mode: AddressingMode::Implied,
        time: 6,
    },

    // RTS
    0x60u8 => Instruction{
        opcode: 0x60,
        mnemonic: Mnemonic::RTS,
        mode: AddressingMode::Implied,
        time: 6,
    },

    // SBC
    0xe9u8 => Instruction{
        opcode: 0xe9,
        mnemonic: Mnemonic::SBC,
        mode: AddressingMode::Immediate,
        time: 2,
    },
    0xe5u8 => Instruction{
        opcode: 0xe5,
        mnemonic: Mnemonic::SBC,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0xf5u8 => Instruction{
        opcode: 0xf5,
        mnemonic: Mnemonic::SBC,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0xedu8 => Instruction{
        opcode: 0xed,
        mnemonic: Mnemonic::SBC,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0xfdu8 => Instruction{
        opcode: 0xfd,
        mnemonic: Mnemonic::SBC,
        mode: AddressingMode::AbsoluteX,
        time: 4,
    },
    0xf9u8 => Instruction{
        opcode: 0xf9,
        mnemonic: Mnemonic::SBC,
        mode: AddressingMode::AbsoluteY,
        time: 4,
    },
    0xe1u8 => Instruction{
        opcode: 0xe1,
        mnemonic: Mnemonic::SBC,
        mode: AddressingMode::IndirectX,
        time: 6,
    },
    0xf1u8 => Instruction{
        opcode: 0xf1,
        mnemonic: Mnemonic::SBC,
        mode: AddressingMode::IndirectY,
        time: 5,
    },

    // STA
    0x85u8 => Instruction{
        opcode: 0x85,
        mnemonic: Mnemonic::STA,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0x95u8 => Instruction{
        opcode: 0x95,
        mnemonic: Mnemonic::STA,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0x8du8 => Instruction{
        opcode: 0x8d,
        mnemonic: Mnemonic::STA,
        mode: AddressingMode::Absolute,
        time: 4,
    },
    0x9du8 => Instruction{
        opcode: 0x9d,
        mnemonic: Mnemonic::STA,
        mode: AddressingMode::AbsoluteX,
        time: 5,
    },
    0x99u8 => Instruction{
        opcode: 0x99,
        mnemonic: Mnemonic::STA,
        mode: AddressingMode::AbsoluteY,
        time: 5,
    },
    0x81u8 => Instruction{
        opcode: 0x81,
        mnemonic: Mnemonic::STA,
        mode: AddressingMode::IndirectX,
        time: 6,
    },
    0x91u8 => Instruction{
        opcode: 0x91,
        mnemonic: Mnemonic::STA,
        mode: AddressingMode::IndirectY,
        time: 6,
    },

    // Stack instructions
    0x9au8 => Instruction{
        opcode: 0x9a,
        mnemonic: Mnemonic::TXS,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0xbau8 => Instruction{
        opcode: 0xba,
        mnemonic: Mnemonic::TSX,
        mode: AddressingMode::Implied,
        time: 2,
    },
    0x48u8 => Instruction{
        opcode: 0x48,
        mnemonic: Mnemonic::PHA,
        mode: AddressingMode::Implied,
        time: 3,
    },
    0x68u8 => Instruction{
        opcode: 0x68,
        mnemonic: Mnemonic::PLA,
        mode: AddressingMode::Implied,
        time: 4,
    },
    0x08u8 => Instruction{
        opcode: 0x08,
        mnemonic: Mnemonic::PHP,
        mode: AddressingMode::Implied,
        time: 3,
    },
    0x28u8 => Instruction{
        opcode: 0x28,
        mnemonic: Mnemonic::PLP,
        mode: AddressingMode::Implied,
        time: 4,
    },

    // STX
    0x86u8 => Instruction{
        opcode: 0x86,
        mnemonic: Mnemonic::STX,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0x96u8 => Instruction{
        opcode: 0x96,
        mnemonic: Mnemonic::STX,
        mode: AddressingMode::ZeroY,
        time: 4,
    },
    0x8eu8 => Instruction{
        opcode: 0x8e,
        mnemonic: Mnemonic::STX,
        mode: AddressingMode::Absolute,
        time: 4,
    },

    // STY
    0x84u8 => Instruction{
        opcode: 0x84,
        mnemonic: Mnemonic::STY,
        mode: AddressingMode::Zero,
        time: 3,
    },
    0x94u8 => Instruction{
        opcode: 0x94,
        mnemonic: Mnemonic::STY,
        mode: AddressingMode::ZeroX,
        time: 4,
    },
    0x8cu8 => Instruction{
        opcode: 0x8c,
        mnemonic: Mnemonic::STY,
        mode: AddressingMode::Absolute,
        time: 4,
    },
};
