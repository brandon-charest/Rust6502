use rust_6502_emulator::hardware::{
    bus::{Bus, Memory},
    cpu::CPU,
};

/// Klaus Dormann's 6502 Functional Test
/// https://github.com/Klaus2m5/6502_65C02_functional_tests
///
/// This test loads the functional test binary and runs it until:
/// - Success: PC gets stuck in an infinite loop (same PC for 2 cycles)
/// - Failure: PC enters a trap (different location, indicating test failure)
///
/// The test binary is pre-assembled to run at address $0400.
/// Success location: $3469 (PC will loop here indefinitely)
#[test]
#[ignore] // Run with: cargo test --test klaus_functional -- --ignored
fn test_klaus_functional() {
    let rom = include_bytes!("roms/6502_functional_test.bin");

    let mut cpu = CPU::new();
    let mut bus = Memory::new();

    // Load the binary into memory at address $0000
    for (i, &byte) in rom.iter().enumerate() {
        bus.write(i as u16, byte);
    }

    // Test starts at $0400
    cpu.registers.program_counter = 0x0400;

    let mut last_pc = cpu.registers.program_counter;
    let mut same_pc_count = 0;
    let max_cycles = 100_000_000; // Prevent infinite loops on failure

    // Track last 20 instructions for debugging
    let mut pc_history: Vec<(usize, u16, u8)> = Vec::with_capacity(20);

    for cycle in 0..max_cycles {
        let current_pc = cpu.registers.program_counter;
        let current_opcode = bus.read(current_pc);

        // Add to history (keep last 20)
        pc_history.push((cycle, current_pc, current_opcode));
        if pc_history.len() > 20 {
            pc_history.remove(0);
        }

        // Success: PC stuck at the same location (success loop)
        if current_pc == last_pc {
            same_pc_count += 1;
            if same_pc_count > 2 {
                // Success location is $3469
                if current_pc == 0x3469 {
                    println!(
                        "✅ SUCCESS! Test passed at PC=${:04X} after {} instructions",
                        current_pc, cycle
                    );
                    return;
                } else {
                    eprintln!(
                        "\n❌ TRAP detected at PC=${:04X} after {} instructions",
                        current_pc, cycle
                    );
                    eprintln!("Expected success at $3469\n");
                    eprintln!("Last 20 instructions before trap:");
                    for (i, pc, op) in &pc_history {
                        eprintln!("  {:06}: PC=${:04X} OP=${:02X}", i, pc, op);
                    }
                    eprintln!("\nCPU State at trap:");
                    eprintln!(
                        "  A=${:02X} X=${:02X} Y=${:02X} SP=${:02X} P={:02X}",
                        cpu.registers.accumulator,
                        cpu.registers.x_register,
                        cpu.registers.y_register,
                        cpu.registers.stack_pointer,
                        cpu.registers.status.bits()
                    );
                    panic!("Test failed - check trap location in Klaus test source");
                }
            }
        } else {
            same_pc_count = 0;
        }

        last_pc = current_pc;
        cpu.step(&mut bus);

        // Optional: Print progress periodically
        if cycle % 10_000_000 == 0 && cycle > 0 {
            println!("Running... {} instructions executed, PC=${:04X}", cycle, current_pc);
        }
    }

    panic!(
        "❌ Test timed out after {} cycles at PC=${:04X}\n\
         Likely missing opcode implementation or infinite loop bug.",
        max_cycles, cpu.registers.program_counter
    );
}

/// Run Klaus test with detailed trace output (useful for debugging)
#[test]
#[ignore] // Run with: cargo test --test klaus_functional -- --ignored trace_klaus
fn trace_klaus_functional() {
    let rom = include_bytes!("roms/6502_functional_test.bin");

    let mut cpu = CPU::new();
    let mut bus = Memory::new();

    for (i, &byte) in rom.iter().enumerate() {
        bus.write(i as u16, byte);
    }

    cpu.registers.program_counter = 0x0400;

    let mut last_pc = cpu.registers.program_counter;
    let mut same_pc_count = 0;
    let max_instructions = 1000; // Limit trace output

    for cycle in 0..max_instructions {
        let current_pc = cpu.registers.program_counter;
        let opcode_byte = bus.read(current_pc);

        println!(
            "{:04}: PC=${:04X} OP=${:02X} A=${:02X} X=${:02X} Y=${:02X} SP=${:02X} P={:02X}",
            cycle,
            current_pc,
            opcode_byte,
            cpu.registers.accumulator,
            cpu.registers.x_register,
            cpu.registers.y_register,
            cpu.registers.stack_pointer,
            cpu.registers.status.bits()
        );

        if current_pc == last_pc {
            same_pc_count += 1;
            if same_pc_count > 2 {
                if current_pc == 0x3469 {
                    println!("✅ SUCCESS!");
                    return;
                } else {
                    panic!("❌ TRAP at ${:04X}", current_pc);
                }
            }
        } else {
            same_pc_count = 0;
        }

        last_pc = current_pc;
        cpu.step(&mut bus);
    }

    println!("Trace stopped after {} instructions", max_instructions);
}
