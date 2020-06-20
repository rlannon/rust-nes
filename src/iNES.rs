// iNES.rs
// Module for reading .nes files and iNES headers

/*

iNES files contain information about mappers, memory sizes, etc.
This emulator will utilize iNES 2.0. It does not necessarily support all features.
The spec can be found at https://wiki.nesdev.com/w/index.php/NES_2.0.

*/

enum Timing {
    NTSC,
    PAL,
    Multi,
    Dendy,
}

pub struct NesFormat {
    // sizes
    prg_rom_size: u16,  // actually 12 bits
    chr_rom_size: u16,  // actually 12 bits

    // shift counts -- note it is "64 << shift_count" to get the size (i.e. 0 = none)
    // this means a shift count of 7 would yield 8192 bytes
    prg_ram_shift_count: u8,    // actually 4 bits
    prg_nvram_shift_count: u8,  // actually 4 bits
    chr_ram_shift_count: u8,    // also 4 bits
    chr_nvram_shift_count: u8,  // also 4 bits

    // flags
    nametable_mirror_type: bool,    // false = horizontal or mapper-controller; true = vertical
    battery_memory_present: bool,
    trainer_present: bool,
    four_screen_mode: bool,
    
    // misc
    mapper_number: u16, // actually 12 bits
    submapper_number: u8,   // actually 4 bits
    timing: Timing,
}

impl NesFormat {
    /// Reads through a binary file (contained within a buffer) and returns a NesFormat object
    pub fn read_ines(buf: &[u8]) -> NesFormat {
        // todo: read file
        NesFormat {
            // dummy values for compilation
            prg_rom_size: 0,
            chr_rom_size: 0,
            prg_ram_shift_count: 0,
            prg_nvram_shift_count: 0,
            chr_ram_shift_count: 0,
            chr_nvram_shift_count: 0,
            nametable_mirror_type: false,
            battery_memory_present: false,
            trainer_present: false,
            four_screen_mode: false,
            mapper_number: 0,
            submapper_number: 0,
            timing: Timing::NTSC,
        }
    }
}
