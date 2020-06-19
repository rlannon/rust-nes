// main.rs

use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io;
use std::io::Write;

pub mod cpu;

fn main() {
    // Create the CPU object
    let mut nes_cpu: cpu::CPU = cpu::CPU::default();
    
    // set up our vectors
    const RESET: u16 = 0x0600;
    const IRQ: u16 = 0x0620;

    // get the program
    print!("Enter the filename (located in samples/): ");
    io::stdout().flush().expect("Flushing output buffer");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read from stdin");
    let filename = format!("samples/{}", s.trim());
    let mut file = File::open(filename).unwrap();
    
    // load the program into memory
    file.read(&mut nes_cpu.memory[RESET as usize..]).unwrap();

    // update the vectors
    nes_cpu.load_vector(cpu::RESET_VECTOR, RESET);
    nes_cpu.load_vector(cpu::IRQ_VECTOR, IRQ);

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
