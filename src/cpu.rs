// cpu.rs
// Implements the 6502 variant used in the NES

mod instruction;

/// The stack page is hard-wired to page 1
const STACK_PAGE: u8 = 0x01;

/// The 6502 has 3 vectors, hard-coded at the end of memory, which contain pointers to various routines. These are necessary for the processor to know where interrupt and reset routines are located. They are:
/// * the NMI vector, located at `0xFFFA - 0xFFFB`;
/// * the reset vector, located at `0xFFFC - 0xFFFD`;
/// * the IRQ vector, located `0xFFFE - 0xFFFF`
/// Since these are hard-coded, we can use named constants for them.
const NMI_VECTOR: u16 = 0xfffa;
const RESET_VECTOR: u16 = 0xfffc;
const IRQ_VECTOR: u16 = 0xfffe;

// Constants for our flag positions
const N_FLAG: u8 = 0b10000000;
const V_FLAG: u8 = 0b01000000;
const B_FLAG: u8 = 0b00010000;
const D_FLAG: u8 = 0b00001000;
const I_FLAG: u8 = 0b00000100;
const Z_FLAG: u8 = 0b00000010;
const C_FLAG: u8 = 0b00000001;

#[derive(PartialEq, Eq)]
enum Flag {
    Negative,
    Overflow,
    B,
    Decimal,
    Interrupt,
    Zero,
    Carry,
}

/// The struct that implements the NES's CPU.
pub struct CPU {
    // track cycle count since last vblank
    cycles: u64,

    // processor registers
    status: u8,
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,

    // processor memory
    memory: [u8; 65536],
}

impl Default for CPU {
    #[inline]
    fn default() -> CPU {
        CPU {
            cycles: 0,
            status: 0,
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            memory: [0; 65536]
        }
    }
}

/// Gets the constant associated with the given Flag
/// For example, if I call `get_flag_constant(Flag::Negative)`, it will return `0b10000000`, or the constant `N_FLAG`.
fn get_flag_constant(f: Flag) -> u8 {
    
    // some arrays to iterate over
    let constants = [N_FLAG, V_FLAG, B_FLAG, D_FLAG, I_FLAG, Z_FLAG, C_FLAG];
    let flags = [
        Flag::Negative,
        Flag::Overflow, 
        Flag::B, Flag::Decimal,
        Flag::Interrupt, 
        Flag::Zero,
        Flag::Carry
    ];
    
    let mut i = 0;
    let mut found = false;
    
    while !found && i < flags.len() {
        if f == flags[i] {
            found = true;
        } else {
            i += 1;
        }
    }
    
    return constants[i];
}

impl CPU {
    /// Sets the register flag `f` to the value `v`
    fn set_flag(&mut self, f: Flag, v: bool) {
        let flag_constant = get_flag_constant(f);
        self.status = (self.status & !flag_constant) | if v { flag_constant } else { 0 };
    }

    /// Returns whether the specified flag is set or not
    fn is_set(&self, f: Flag) -> bool {
        let flag_constant = get_flag_constant(f);
        return (self.status & flag_constant) != 0;
    }

    /// Reads an 8-bit value for a register load according to the addressing mode
    /// This function automatically reads the appropriate number of bytes and updates the status register according to the value read
    fn read_value(&mut self, mode: instruction::AddressingMode) -> u8 {
        // Get the offset
        let offset = if 
            mode == instruction::AddressingMode::AbsoluteX || 
            mode == instruction::AddressingMode::IndirectX || 
            mode == instruction::AddressingMode::ZeroX {
            self.x
        } else if 
            mode == instruction::AddressingMode::AbsoluteY ||
            mode == instruction::AddressingMode::IndirectY ||
            mode == instruction::AddressingMode::ZeroY {
            self.y
        } else {
            0
        };

        let value: u8;

        // Get the value
        if mode == instruction::AddressingMode::Immediate {
            value = self.memory[self.pc as usize];
        }
        else if
            mode == instruction::AddressingMode::Zero ||
            mode == instruction::AddressingMode::ZeroX ||
            mode == instruction::AddressingMode::ZeroY {
                let address: u8 = self.memory[self.pc as usize] + offset;
                value = self.memory[address as usize];
        }
        else if
            mode == instruction::AddressingMode::Absolute ||
            mode == instruction::AddressingMode::AbsoluteX ||
            mode == instruction::AddressingMode::AbsoluteY {
                let address: u16 = self.read_absolute_address() + offset as u16;
                value = self.memory[address as usize];
        }
        else if mode == instruction::AddressingMode::IndirectX {
            let address: u16 = self.read_indexed_indirect_address();
            value = self.memory[address as usize];
        }
        else if mode == instruction::AddressingMode::IndirectY {
            let address: u16 = self.read_indirect_indexed_address();
            value = self.memory[address as usize];
        }
        else {
            // invalid addressing mode
            // todo: exception of some sort?
            value = 0;
        }

        value
    }

    /// Update the status register based on a given value
    /// This only affects the Z and N flags
    fn update_status(&mut self, value: u8) {
        if value == 0 {
            self.set_flag(Flag::Zero, true);
            self.set_flag(Flag::Negative, false);
        }
        else
        {
            self.set_flag(Flag::Zero, false);
            
            if value > 127 {
                self.set_flag(Flag::Negative, true);
            }
        }
    }

    /// Get the address located at self.pc, self.pc + 1
    /// Increments the pc to the last byte of the address
    fn read_absolute_address(&mut self) -> u16 {
        let address =
            (self.memory[self.pc as usize] as u16) |
            ((self.memory[(self.pc + 1) as usize] as u16) << 8);
        self.pc += 1;
        return address;
    }

    /// Gets an indirect address
    /// Indirect addresses always give the first byte of the pointer, meaning if the value `0x23C0` is given, it looks to `0x23C0 - 0x23C1` for the address.
    ///
    /// This function reproduces the behavior of a well-known hardware bug of the 6502 that is caused when the low byte of the address is located on the last byte of a page. When this happens, the full 16-bit address is not incremented by one, rather, *only the low byte* is. This means if we have an instruction like
    ///
    ///     jmp ($02FF)
    ///
    /// instead of loading the address from `0x02FF - 0x0300`, the low byte will come from `0x02FF` and the high byte will come from `0x0200`. As such, an indirect jump should *never* use the last byte of a page in its indirection.
    fn read_indirect_address(&mut self) -> u16 {
        // fetch the address locations
        let ptr_low: u8 = self.memory[self.pc as usize];
        self.pc += 1;
        let mut ptr_high: u8 = self.memory[self.pc as usize];

        // construct the indirection
        let addr_low: u8 = self.memory[
            (((ptr_high as u16) << 8) | 
            (ptr_low as u16))
            as usize
        ];
        ptr_high += 1;  // if it is 0xff, it will wrap around
        let addr_high: u8 = self.memory[
            (((ptr_high as u16) << 8) | 
            (ptr_low as u16))
            as usize
        ];

        // return the address
        return (addr_high as u16) << 8 | addr_low as u16;
    }

    /// Gets the address for the indirect indexed (indirect Y) addressing mode
    /// Reads one byte, giving the address in the zero page where the pointer is stored; the little-endian 16-bit address is then read and returned
    /// Since indirect indexed can only be used with the Y register, we don't need an offset
    fn read_indirect_indexed_address(&self) -> u16 {
        let zp_address: u8 = self.memory[self.pc as usize];
        let mut address: u16 = 
            (self.memory[zp_address as usize] as u16) |
            ((self.memory[(zp_address + 1) as usize] as u16) << 8)
        ;
        address += self.y as u16;

        address
    }

    /// Gets the indexed indirect address (indirect X)
    /// Like indirect indexed, indexed indirect can only be used with the X register -- so we don't need an offset
    fn read_indexed_indirect_address(&self) -> u16 {
        let zp_address: u8 = self.memory[self.pc as usize] + self.x;
        let address: u16 =
            (self.memory[zp_address as usize] as u16) |
            ((self.memory[(zp_address + 1) as usize] as u16) << 8);
        address
    }

    /// Store an 8-bit value `value` in memory at address according to the addressing mode `mode`.
    /// Affects no flags.
    fn store(&mut self, value: u8, mode: instruction::AddressingMode) {
        // Get the offset
        let offset: u8 = 
                if mode == instruction::AddressingMode::AbsoluteX ||
                    mode == instruction::AddressingMode::ZeroX
                {
                    self.x
                }
                else if mode == instruction::AddressingMode::AbsoluteY ||
                        mode == instruction::AddressingMode::AbsoluteY
                {
                    self.y
                }
                else {
                    0
                };
        
        // Store according to the mode
        let address;
        if mode == instruction::AddressingMode::Absolute || mode == instruction::AddressingMode::AbsoluteX || mode == instruction::AddressingMode::AbsoluteY {
            address = self.read_absolute_address() + offset as u16;
        }
        else if mode == instruction::AddressingMode::Zero || mode == instruction::AddressingMode::ZeroX || mode == instruction::AddressingMode::ZeroY {
            address = (self.memory[self.pc as usize] + offset) as u16;
        }
        else if mode == instruction::AddressingMode::IndirectX {
            address = self.read_indexed_indirect_address();
        } else if mode == instruction::AddressingMode::IndirectY {
            address = self.read_indirect_indexed_address();
        }
        else {
            // todo: invalid mode?
            address = 0;
        }

        // perform the assignment
        self.memory[address as usize] = value;
    }

    /// Fetch a value from memory
    fn fetch(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    /// Push a value `value` onto the stack
    /// Note this will increment the SP and *then* write the value
    /// It's also worth noting that the 6502 does not have overflow detection, so if the stack pointer wraps around, that's normal behavior for the processor
    fn push(&mut self, value: u8) {
        self.sp += 1;
        let address: u16 = ((STACK_PAGE as u16) << 8) | (self.sp as u16);
        self.memory[address as usize] = value;
    }

    /// Pop a value off the stack
    /// This will read the value and then decrement the SP
    /// Keep in mind, as with the `push` function, the 6502 does not have stack underflow detection
    fn pop(&mut self) -> u8 {
        let address: u16 = ((STACK_PAGE as u16) << 8) | (self.sp as u16);
        let value = self.memory[address as usize];
        self.sp -= 1;
        return value;
    }

    /// Performs subtraction, fetching values automatically according to `mode`. Also automatically stores result in the accumulator.
    fn sbc(&mut self, mode: instruction::AddressingMode) {
        // fetch our values
        let minuend = self.a as u16 | if self.is_set(Flag::Carry) { 0x100 } else { 0 };
        let subtrahend = self.read_value(mode);

        // perform the subtraction
        let result = minuend - subtrahend as u16;
        self.set_flag(
            Flag::Carry, 
            if result & 0x100 != 0 { true } else { false }
        );

        // todo: update flags

        // finally, set A
        self.a = result as u8;
    }

    /// Performs addition, fetching values automatically according to `mode`. Also automatically stores result in the accumulator.
    fn adc(&mut self, mode: instruction::AddressingMode) {
        // fetch values
        let addend = self.a as u16;
        let augend = self.read_value(mode) as u16;
        
        // set the overflow flag if necessary (addition would take it out of the signed integer range)
        self.set_flag(
            Flag::Overflow, 
            if (addend ^ augend) & 0x80 != 0 { false } else { true }
        );
        
        let result: u16 = addend + augend + if self.is_set(Flag::Carry) { 1 } else { 0 };
    }

    /// Executes the instruction supplied; reads from memory appropriately
    fn execute_instruction(&mut self, opcode: u8) {
        // todo: this can be optimized with a few getter functions and instruction lookups

        let i: instruction::Instruction = instruction::INSTRUCTIONS[&opcode];

        if i.mnemonic == instruction::Mnemonic::ADC {
            self.adc(i.mode);
        }

        // todo: more opcodes

        else if opcode == 0x4c {
            // JMP - Absolute
            self.pc = self.read_absolute_address();
        }
        else if opcode == 0x6c {
            // JMP - Indirect
            self.pc = self.read_indirect_address();
        }
        else if i.mnemonic == instruction::Mnemonic::LDA {
            // LDA
            self.a = self.read_value(i.mode);
            self.update_status(self.a);
        }
        else if i.mnemonic == instruction::Mnemonic::LDX {
            // LDX
            self.x = self.read_value(i.mode);
            self.update_status(self.x);
        }
        else if i.mnemonic == instruction::Mnemonic::LDY {
            // LDY
            self.y = self.read_value(i.mode);
            self.update_status(self.y);
        }

        /*

        Note that the SBC instruction normally can utilize binary-coded decimal,
            but the decimal flag is not functional on the NES
        As such, these instructions never utilize BCD

        The carry flag is always used in subtraction - there is no way to subtract without carry


        */
        else if i.mnemonic == instruction::Mnemonic::SBC {
            // SBC, Imm
            self.sbc(i.mode);
        }
        else if i.mnemonic == instruction::Mnemonic::STA {
            // STA - ZP
            self.store(self.a, i.mode);
        }
        else if opcode == 0x9a {
            // TXS
            self.sp = self.x;
            self.update_status(self.sp);
        }
        else if opcode == 0xba {
            // TSX
            self.x = self.sp;
            self.update_status(self.x);
        }
        else if opcode == 0x48 {
            // PHA
            self.push(self.a);
        }
        else if opcode == 0x68 {
            // PLA
            self.a = self.pop();
            self.update_status(self.a);
        }
        else if opcode == 0x08 {
            // PHP
            self.push(self.status);
        }
        else if opcode == 0x28 {
            // PLP
            self.status = self.pop();
        }
        else if i.mnemonic == instruction::Mnemonic::STX {
            // STX
            self.store(self.x, i.mode);
        }
        else if i.mnemonic == instruction::Mnemonic::STY {
            // STY
            self.store(self.y, i.mode);
        }
    }

    /// Steps the processor, executing an instruction
    pub fn step(&mut self) {
        // fetch the byte at the address indicated by the pc
        let instruction = self.memory[self.pc as usize];
        self.pc += 1;   // increment the pc by one during the 'fetch cycle'
        
        // execute that instruction
        self.execute_instruction(instruction);
        
        // increment the program counter
        self.pc += 1;
    }

    /// Resets the CPU, leaving it in a ready state
    pub fn reset(&mut self) {
        // get the start address
        // remember, the 6502 is little endian, so we fetch the high byte, then the low byte
        self.pc = RESET_VECTOR;
        let start_address: u16 = self.read_absolute_address();
        self.pc = start_address;

        // todo: additional start routines
    }
}
