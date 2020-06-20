// nes.rs
// Implements the NES functionality, bringing together the CPU, PPU, and APU

use Box;
use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::cpu;
use crate::ppu;

// constants for clock speeds
const MASTER_CLOCK_RATE: u32 = 21_477_272;
const VBLANK_RATE: u32 = MASTER_CLOCK_RATE / 60;    // vlbank happens every 60th of a second (about once every 17 milliseconds)

// our clock factors
const CPU_CLOCK_FACTOR: u32 = 12;    // the CPU clock is 1/12 the master
const PPU_CLOCK_FACTOR: u32 = 4;     // the PPU clock is 1/4 the master
const APU_BLOCK_FACTOR: u32 = 24;    // the APU clock is 1/24 the master

// pre-compute clock rates
const CPU_CLOCK_RATE: u32 =  MASTER_CLOCK_RATE / CPU_CLOCK_FACTOR;
const PPU_CLOCK_RATE: u32 = MASTER_CLOCK_RATE / PPU_CLOCK_FACTOR;
const APU_CLOCK_RATE: u32 = MASTER_CLOCK_RATE / APU_BLOCK_FACTOR;

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

    /// Loads a program into memory and executes it.
    /// The program will be loaded according to the mapper it uses.
    pub fn run(&mut self) {
        // todo: mappers

        let mut start_instant = Instant::now();
        let mut do_update = false;

        while self.cpu.is_running() {
            // update the time if we need to
            if do_update {
                start_instant = Instant::now();
                do_update = false;
            }
            else {
                // make sure the number of master cycles is lower than the number in a second
                if self.cycles < MASTER_CLOCK_RATE {
                    // todo: if it's time for vblank, signal the cpu 

                    if CPU_CLOCK_RATE >= self.cpu.cycle_count() {
                        self.cpu.step();
                    }
                    // todo: update PPU, APU

                    self.cycles += 4;   // update the number of cycles that have passed
                                        // note we are using 4 because that's the fewest that can pass with one tick;
                                        // the PPU is the fastest element and runs 1/4 the rate of the master 
                }
                else {
                    println!("Sleeping for duration; cycles passed: {}", self.cycles);
                    let second = Duration::new(1, 0);
                    sleep(second - start_instant.elapsed());
                    do_update = true;
                }
            }
        }
    }
}
