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
    file.read(&mut nes_cpu.memory[0x0600..]).unwrap();

    // update the vectors
    nes_cpu.memory[cpu::RESET_VECTOR as usize] = 0x00;
    nes_cpu.memory[cpu::RESET_VECTOR as usize + 1] = 0x06;
    nes_cpu.memory[cpu::IRQ_VECTOR as usize] = 0x20;
    nes_cpu.memory[cpu::IRQ_VECTOR as usize + 1] = 0x06;

    // reset the system
    nes_cpu.reset();

    // maintain an accurate speed
    let emu_speed = cpu::NTSC_SPEED as u64; // depends on whether it is running in NTSC or PAL mode
    let mut now = Instant::now();
    let mut update = false;

    // run the program
    while nes_cpu.is_running() {
        if update {
            now = Instant::now();
            update = false;
        }

        if nes_cpu.cycle_count() < emu_speed {
            nes_cpu.step();
        } else {
            println!("Cycles passed: {}", nes_cpu.cycle_count());
            let second = Duration::new(1, 0);
            sleep(second - now.elapsed());
            update = true;
        }
    }

    // print info on exit
    nes_cpu.print_cpu_information();
}
