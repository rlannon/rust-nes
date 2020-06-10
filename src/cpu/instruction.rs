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
    Single,
    Relative,
}

struct Instruction {
    opcode: u8,
    mnemonic: Mnemonic,
    mode: AddressingMode,
}

static INSTRUCTIONS: phf::Map<u8, Instruction> = phf_map! {
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
};
