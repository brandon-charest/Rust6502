mod hardware;
use hardware::cpu::CPU;

fn main() {
    println!("Rust6502 Emulator");
    let mut cpu = CPU::new();

    println!("CPU: {:#?}", cpu);
}
