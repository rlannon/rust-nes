// cpu.rs
// Implements the 6502 variant used in the NES

use std::collections::HashMap;
use maplit::hashmap;

mod cpu {
    /// The stack page is hard-wired to page 1
    const stack_page: u8 = 0x01;

    /// The 6502 has 3 vectors located at the end of memory
    const nmi_vector: u16 = 0xfffa;
    const reset_vector: u16 = 0xfffc;
    const irq_vector: u16 = 0xfffe;

    const n_flag: u8 = 0b10000000;
    const v_flag: u8 = 0b01000000;
    const b_flag: u8 = 0b00010000;
    const d_flag: u8 = 0b00001000;
    const i_flag: u8 = 0b00000100;
    const z_flag: u8 = 0b00000010;
    const c_flag: u8 = 0b00000001;

    enum Flag {
        Negative,
        Overflow,
        B,
        Decimal,
        Interrupt,
        Zero,
        Carry,
    }

    enum AddressingMode {
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

    /// Data about a 6502 instruction
    /// All 6502 instructions
    struct Instruction {
        opcode: u8,
        addressing_mode: AddressingMode,
        length: u8,
        time: u8,
        page_boundary_increase: bool,
    }

    const all_instructions = hashmap! {
        // JMP
        0x4c: u8 => Instruction{
            opcode: 0x4c,
            addressing_mode: AddressingMode::Absolute,
            length: 3,
            time: 3,
            page_boundary_increase: false,
        },
        0x6c: u8 => Instruction{
            opcode: 0x6c,
            addressing_mode: AddressingMode::Indirect,
            length: 3,
            time: 5,
            page_boundary_increase: false,
        },
        // JSR
        0x20: u8 => Instruction{
            opcode: 0x20,
            addressing_mode: AddressingMode::Absolute,
            length: 3,
            time: 6,
            page_boundary_increase: false,
        }
        // LDA
        0xa9: u8 => Instruction{
            opcode: 0xa9,
            addressing_mode: AddressingMode::Immediate,
            length: 2,
            time: 2,
            page_boundary_increase: false,
        },
        0xa5: u8 => Instruction{
            opcode: 0xa5,
            addressing_mode: AddressingMode::Zero,
            length: 2,
            time: 3,
            page_boundary_increase: false,
        },
        0xb5: u8 => Instruction{
            opcode: 0xb5,
            addressing_mode: AddressingMode::ZeroX,
            length: 2,
            time: 4,
            page_boundary_increase: false,
        },
        0xad: u8 => Instruction{
            opcode: 0xad,
            addressing_mode: AddressingMode::Absolute,
            length: 3,
            time: 4,
            page_boundary_increase: false,
        },
        0xbd: u8 => Instruction {
            opcode: 0xbd,
            addressing_mode: AddressingMode::AbsoluteX,
            length: 3,
            time: 4,
            page_boundary_increase: true,
        },
        0xb9: u8 => Instruction {
            opcode: 0xb9,
            addressing_mode: AddressingMode::AbsoluteY,
            length: 3,
            time: 4,
            page_boundary_increase: true,
        },
    }

    /// The struct that implements the NES's CPU
    pub struct CPU {
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

    fn get_flag_constant(f: Flag) -> u8 {
        /// Gets the constant for the Flag
        
        // some arrays to iterate over
        let constants = [n_flag, v_flag, b_flag, d_flag, i_flag, z_flag, c_flag];
        let flags = [Negative, Overflow, B, Decimal, Interrupt, Zero, Carry];
        
        let mut i = 0;
        let mut found = false;
        
        while (!found && i < flags.len()) {
            if (f == flags[i]) {
                found = true;
            } else {
                i += 1;
            }
        }
        
        return constants[i];
    }

    impl CPU {
        /// Gets the status register byte
        /// Note the flags are ordered as follows:
        /// `N V - B D I Z C`
        fn get_status(&self) -> &u8 {
            return &self.status;
        }

        fn set_flag(&self, f: Flag, v: bool) {
            let flag_constant = get_flag_constant(f);
            self.status = (self.status & !flag_constant) | if v { flag_constant } else { 0 };
        }

        /// Update the status register based on a given value
        fn update_status(&self, value: u8) {
            if value == 0 {
                self.set_flag(Zero, true);
                self.set_flag(Negative, false);
            }
            else
            {
                self.set_flag(Zero, false);
                
                if value > 127 {
                    self.set_flag(Negative, true);
                }
            }
        }

        /// Get the address located at self.pc, self.pc + 1
        /// Increments the pc to the last byte of the address
        fn read_absolute_address(&self) -> u16 {
            let address = (
                (self.memory[self.pc as usize] as u16) |
                ((self.memory[(self.pc + 1) as usize] << 8) as u16)
            );
            self.pc += 1;
            return address;
        }

        /// Store an 8-bit value `value` in memory at address `address` 
        fn store(&self, address: u16, value: u8) {
            self.memory[address as usize] = value;
        }

        /// Fetch a value from memory
        fn fetch(&self, address: u16) -> u8 {
            return self.memory[address as usize];
        }

        /// Push a value `value` onto the stack
        /// Note this will increment the SP and *then* write the value
        fn push(&self, value: u8) {
            self.sp += 1;
            let address: u16 = ((stack_page as u16) << 8) | (self.sp as u16);
            self.memory[address as usize] = value;
        }

        /// Pop a value off the stack
        /// This will read the value and then decrement the SP
        fn pop(&self) -> u8 {
            let address: u16 = ((stack_page << 8) as u16) | (self.sp as u16);
            let value = self.memory[address as usize];
            self.sp -= 1;
            return value;
        }

        /// Executes the instruction supplied; reads from memory appropriately
        fn execute_instruction(&self, opcode: u8) {
            if opcode == 0x4c {
                // JMP - Absolute
                self.pc = self.read_absolute_address();
            }
            else if opcode == 0xa9 {
                // LDA - Immediate
                // fetch the immediate value
                self.a = self.memory[self.pc as usize];
                self.pc += 1;
            }
            else if opcode == 0xa5 {
                // LDA - Zero
                let address: u8 = self.memory[self.pc as usize];
                self.a = self.memory[address as usize];
                self.pc += 1;
            }
            else if opcode == 0xb5 {
                // LDA - Zero, X
                // offset with the X register
                let address: u8 = self.memory[self.pc as usize] + self.x;
                self.a = self.memory[address as usize];
                self.pc += 1;
            }
            else if opcode == 0xad {
                // LDA - Absolute
                let address: u16 = self.read_absolute_address();
                self.a = self.memory[address as usize];
                self.pc += 1;
            }
            else if opcode == 0xbd {
                // LDA - Absolute, X
                let address: u16 = self.read_absolute_address() + self.x;
                self.a = self.memory[address as usize];
                self.pc += 1;
            }
            else if opcode == 0xb9 {
                // LDA - Absolute, Y
                let address: u16 = self.read_absolute_address() + self.y;
                self.a = self.memory[address as usize];
                self.pc += 1;
            }
            else if opcode == 0xa1 {
                // LDA - Indirect, X
            }
            else if opcode == 0xb1 {
                // LDA - Indirect, Y
            }
        }

        /// Steps the processor, executing an instruction
        pub fn step(&self) {
            // fetch the byte at the address indicated by the pc
            let instruction = self.memory[self.pc as usize];
            // increment the pc by one during the 'fetch cycle'
            self.pc += 1;
            // execute that instruction
            self.execute_instruction(instruction);
            
            // increment the program counter
            self.pc += 1;
        }

        /// Start CPU execution
        pub fn start(&self) {
            // get the start address
            // remember, the 6502 is little endian, so we fetch the high byte, then the low byte
            self.pc = reset_vector;
            let start_address: u16 = self.read_absolute_address();
            self.pc = start_address;

            // todo: additional start routines
        }
    }
}
