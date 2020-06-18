// main.rs

use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use std::thread::sleep;

pub mod cpu;

fn main() {
    // Create the CPU object
    let mut nes_cpu: cpu::CPU = cpu::CPU::default();
    
    // load the program into memory at location 0x6000
    let mut file = File::open("samples/test.bin").unwrap();
    file.read(&mut nes_cpu.memory[0x6000..]).unwrap();

    // update the reset vector
    nes_cpu.memory[cpu::RESET_VECTOR as usize] = 0x00;
    nes_cpu.memory[cpu::RESET_VECTOR as usize + 1] = 0x60;

    // reset the system, starting execution
    nes_cpu.reset();

    // maintain an accurate speed
    let mut now = Instant::now();
    let mut update = false;
    while nes_cpu.running() {
        if update {
            now = Instant::now();
        }

        if nes_cpu.cycle_count() < cpu::NTSC_SPEED as u64 {
            nes_cpu.step();
        } else {
            println!("Cycles passed: {}", nes_cpu.cycle_count());
            nes_cpu.print_cpu_information();
            println!("----------------");
            let second = Duration::new(1, 0);
            sleep(second - now.elapsed());
            update = true;
        }
    }
}
