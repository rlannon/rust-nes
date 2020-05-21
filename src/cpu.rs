// cpu.rs
// Implements the 6502 variant used in the NES

mod cpu {
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
        
        // stack address
        stack_page: u8,

        // vector constants -- these take up two bytes each
        nmi_vector: u16,
        reset_vector: u16,
        irq_vector: u16,
    }

    impl CPU {
        fn get_status(&self) -> &u8 {
            return &self.status;
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
            let address: u16 = ((self.stack_page as u16) << 8) | (self.sp as u16);
            self.memory[address as usize] = value;
        }

        /// Pop a value off the stack
        /// This will read the value and then decrement the SP
        fn pop(&self) -> u8 {
            let address: u16 = ((self.stack_page << 8) as u16) | (self.sp as u16);
            let value = self.memory[address as usize];
            self.sp -= 1;
            return value;
        }

        /// Executes the instruction supplied; reads from memory appropriately
        fn execute_instruction(&self, instruction: u8) {
            // todo: implement
        }

        /// Steps the processor, executing an instruction
        pub fn step(&self) {
            // fetch the byte at the address indicated by the pc
            let instruction = self.memory[self.pc as usize];
            self.execute_instruction(instruction);

            // todo: clean-up
        }

        /// Start CPU execution
        pub fn start(&self) {
            // get the start address
            let start_address: u16 = (
                (self.memory[self.reset_vector as usize] << 8) as u16) | 
                (self.memory[(self.reset_vector + 1) as usize] as u16);
            self.pc = start_address;

            // todo: additional start routines
        }
    }
}
