mod hardware;
use hardware::cpu::CPU;

use crate::hardware::{
    bus::{Bus, Memory},
    status::{Status, StatusArgs},
};

fn main() {
    let mut cpu = CPU::new();
    let mut bus = Memory::new();
    #[rustfmt::skip]
    let program: Vec<u8> = vec![
        0xA9, 0x01,        // 8000: LDA #$01   (Set A = 1)
        0x4C, 0x07, 0x80,  // 8002: JMP $8007  (Jump over the next instruction)
        0xA9, 0x00,        // 8005: LDA #$00   (TRAP! If we run this, JMP failed)
        0xA9, 0xFF,        // 8007: LDA #$FF   (Success! Set A = 255)
    ];

    for (i, &byte) in program.iter().enumerate() {
        bus.write(0x8000 + i as u16, byte);
    }
    bus.write(0xFFFC, 0x00);
    bus.write(0xFFFD, 0x80);
    cpu.reset(&mut bus);

    println!("--- Starting 6502 Emulation ---");
    for _ in program.iter() {
        cpu.debug_info(&mut bus);

        cpu.step(&mut bus);
        if cpu.registers.status.contains(Status::BRK) {
            println!("\n--- BRK Encountered! Execution Stopped. ---");
            println!("Final Accumulator: {:02X} (Should be FF)", cpu.registers.accumulator);
            break;
        }
    }

    println!("--- Emulation Finished ---");
}
