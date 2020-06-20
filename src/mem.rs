// mem.rs
// Implements the Memory trait

/// Implements memory functionality.
/// This is to be implemented on each element of the NES that accesses memory.
pub trait Mem {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address:u16, value: u8);
}

/// CPU memory implementation
/// 
/// Due to the way the CPU addresses memory, it mirrors the memory at `$0000 - $07FF` starting at 
/// `$0800 - $0FFF` all the way to `$1800 - $1FFF`. These functions will automatically adjust the address
/// so that valid memory is accessed.
///
/// However, memory from `$6000 - $7FFF` is valid (it is the system's SRAM), so we will mask against `$67FF`.
pub struct CpuRam {
    data: [u8; 0x800],  // the NES only has 2kb of RAM
}

impl Mem for CpuRam {
    fn read(&self, address: u16) -> u8 {
        self.data[(address & 0x67ff) as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[(address & 0x67ff) as usize] = value;
    }
}

impl CpuRam {
    pub fn new() -> CpuRam {
        CpuRam {
            data: [0; 0x800],
        }
    }
}
