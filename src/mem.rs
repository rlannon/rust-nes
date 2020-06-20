// mem.rs
// Implements the Memory trait

/// Implements memory functionality.
/// This is to be implemented on each element of the NES that accesses memory.
pub trait Mem {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address:u16, value: u8);
}

/// CPU memory
pub struct CpuRam {
    data: [u8; 0x800],  // the NES only has 2kb of RAM
}

impl Mem for CpuRam {
    fn read(&self, address: u16) -> u8 {
        self.data[(address & 0x7ff) as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[(address & 0x7ff) as usize] = value;
    }
}

impl CpuRam {
    pub fn new() -> CpuRam {
        CpuRam {
            data: [0; 0x800],
        }
    }
}