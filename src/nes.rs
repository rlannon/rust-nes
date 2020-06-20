// nes.rs
// Implements the NES functionality, bringing together the CPU, PPU, and APU

use Box;

use crate::cpu;
use crate::ppu;

// constants for clock speeds
const MASTER_CLOCK: u32 = 21_477_272;
const VBLANK_RATE: u32 = MASTER_CLOCK / 60;    // vlbank happens every 60th of a second (about once every 17 milliseconds)
const CPU_CLOCK_FACTOR: u8 = 12;    // the CPU clock is 1/12 the master
const PPU_CLOCK_FACTOR: u8 = 4;     // the PPU clock is 1/4 the master
const APU_BLOCK_FACTOR: u8 = 24;    // the APU clock is 1/24 the master

pub struct NES {
    // processors within the system
    pub(in crate) ppu: Box<ppu::PPU>,
    pub(in crate) cpu: Box<cpu::CPU>,

    // the total number of cycles passed -- to keep everything in sync and running at the proper speed
    cycles: u32,
}

impl NES {
    pub fn new() -> NES {
        let mut ppu = Box::new(
            ppu::PPU::new()
        );
        let mut cpu = Box::new(
            cpu::CPU::new(
                ppu.ppuctrl as *mut u8,
                ppu.ppumask as *mut u8,
                ppu.ppustatus as *mut u8,
                ppu.oamaddr as *mut u8,
                ppu.oamdata as *mut u8,
                ppu.ppuscroll as *mut u8,
                ppu.ppuaddr as *mut u8,
                ppu.ppudata as *mut u8,
                ppu.oamdata as *mut u8
            )
        );

        NES {
            ppu: ppu,
            cpu: cpu,

            cycles: 0,
        }
    }
}
