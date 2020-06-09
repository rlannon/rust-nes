// main.rs

pub mod cpu;

fn main() {
    // todo: everything
    let mut nes_cpu: cpu::CPU = cpu::CPU::default();
    nes_cpu.reset();
}
