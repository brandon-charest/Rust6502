mod hardware;
use hardware::cpu::CPU;

use crate::hardware::bus::{Bus, Memory};

fn main() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    memory.write(0x8000, 0xEA); // NOP
    memory.write(0x8001, 0xEA); // NOP
    memory.write(0x8002, 0xEA); // NOP
    memory.write(0x8003, 0x00); //treat 0x00 as a BRK

    memory.write(0xFFFC, 0x00);
    memory.write(0xFFFD, 0x80);

    cpu.reset(&mut memory);
    println!("--- Starting 6502 Emulation ---");
    loop {
        cpu.debug_info(&memory);

        let current_opcode = memory.read(cpu.registers.program_counter);
        if current_opcode == 0x00 {
            println!("BRK (0x00) detected. Halting.");
            break;
        }

        cpu.step(&mut memory);
    }

    println!("--- Emulation Finished ---");
}
