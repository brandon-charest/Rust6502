use crate::hardware::{bus::Memory, status::Status};

use super::*;

#[test]
fn test_cpu_new() {
    let cpu = CPU::new();
    assert_eq!(cpu.registers.accumulator, 0);
    assert_eq!(cpu.registers.x_register, 0);
    assert_eq!(cpu.registers.y_register, 0);
    assert_eq!(cpu.registers.stack_pointer, 0xFD);
    assert_eq!(cpu.registers.program_counter, 0);
    assert_eq!(cpu.registers.status, Status::default());

    assert_eq!(cpu.cycles, 0);
    assert_eq!(cpu.halted, false);
}

#[test]
fn test_cpu_reset() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0xFFFC, 0x00);
    bus.write(0xFFFD, 0x80);

    cpu.reset(&mut bus);

    assert_eq!(cpu.registers.program_counter, 0x8000);
    assert_eq!(cpu.registers.stack_pointer, 0xFD);
    assert!(cpu.registers.status.contains(Status::DISABLE_INTERRUPTS));
}

#[test]
fn test_fetch_byte() {
    let bus = &mut Memory::new();
    let mut cpu = CPU::new();

    // Put an opcode at 0x8000
    let pc_start = 0x8000;
    cpu.registers.program_counter = pc_start;
    bus.write(pc_start, 0xEA); // NOP instruction

    let opcode = cpu.fetch_byte(bus);

    assert_eq!(opcode, 0xEA);
    assert_eq!(cpu.registers.program_counter, pc_start + 1);
}

#[test]
fn test_fetch_u16() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x2000, 0x34);
    bus.write(0x2001, 0x12);

    let value = cpu.read_word(&mut bus, 0x2000);

    assert_eq!(value, 0x1234);
}

#[test]
fn test_fetch_u16_wrapping() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0xFFFF, 0xAA);
    bus.write(0x0000, 0xBB);

    let value = cpu.read_word(&mut bus, 0xFFFF);

    assert_eq!(value, 0xBBAA);
}

#[test]
fn test_lda_immediate_timing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Program: LDA $10 (Immediate)
    bus.write(0x8000, 0xA9); // Opcode (Cycle 1)
    bus.write(0x8001, 0x10); // Operand: $10 (The value 16)

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.cycles, 2, "Immediate should take exactly 2 cycles");
    assert_eq!(cpu.registers.program_counter, 0x8002);
    assert_eq!(cpu.registers.accumulator, 0x10);
}

#[test]
fn test_lda_zeropage_timing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Program: LDA $44 (Zero Page)
    bus.write(0x8000, 0xA5); // Opcode (Cycle 1)
    bus.write(0x8001, 0x44); // Address $44 (Cycle 2)
    bus.write(0x0044, 0x55); // Data at target (Cycle 3)

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.cycles, 3, "Zero Page should take exactly 3 cycles");
    assert_eq!(cpu.registers.program_counter, 0x8002);
    assert_eq!(cpu.registers.accumulator, 0x55);
}

#[test]
fn test_lda_zeropage_x_timing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Program: LDA $44 (Zero Page X)
    bus.write(0x8000, 0xB5);
    bus.write(0x8001, 0x44);
    bus.write(0x0049, 0xAF);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x05;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.cycles, 4, "Zero Page should take exactly 4 cycles");
    assert_eq!(cpu.registers.program_counter, 0x8002);
    assert_eq!(cpu.registers.accumulator, 0xAF);
}

#[test]
fn test_lda_absolute() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA $1234
    bus.write(0x8000, 0xAD); // Opcode
    bus.write(0x8001, 0x34); // Low Byte
    bus.write(0x8002, 0x12); // High Byte
    bus.write(0x1234, 0x55);

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.cycles, 4, "Absolute should take exactly 4 cycles");
    assert_eq!(cpu.registers.program_counter, 0x8003);
    assert_eq!(cpu.registers.accumulator, 0x55);
}

#[test]
fn test_lda_absolute_x_no_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA $1000, X
    bus.write(0x8000, 0xBD);
    bus.write(0x8001, 0x00); // Low
    bus.write(0x8002, 0x10); // High
    bus.write(0x1005, 0xCD);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x05;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0xCD);
    assert_eq!(cpu.cycles, 4, "Absolute,X (No Cross) should take 4 cycles");
}

#[test]
fn test_lda_absolute_x_page_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA $10FF, X
    bus.write(0x8000, 0xBD);
    bus.write(0x8001, 0xFF);
    bus.write(0x8002, 0x10);
    bus.write(0x1100, 0x77);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x01;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x77);
    assert_eq!(cpu.cycles, 5, "Absolute,X (Cross) should take 5 cycles");
}

// Base: $2080, Y: $80 -> Target: $2100 ($2080 + $80 = $2100)
#[test]
fn test_lda_absolute_y_page_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x8000, 0xB9); // LDA Absolute,Y
    bus.write(0x8001, 0x80);
    bus.write(0x8002, 0x20);

    bus.write(0x2100, 0x88);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.y_register = 0x80;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x88);
    assert_eq!(cpu.cycles, 5, "Absolute,Y (Cross) should take 5 cycles");
}

#[test]
fn test_lda_indirect_x() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA ($20, X)
    bus.write(0x8000, 0xA1);
    bus.write(0x8001, 0x20);
    bus.write(0x0024, 0x34);
    bus.write(0x0025, 0x12);

    // Put the value 0x99 at $1234
    bus.write(0x1234, 0x99);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x04;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x99);
    assert_eq!(cpu.cycles, 6, "Indirect X should take 6 cycles");
}

#[test]
fn test_lda_indirect_y_no_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // DA ($20), Y
    bus.write(0x8000, 0xB1);
    bus.write(0x8001, 0x20); // Base ZP address

    // At $0020, store the address $1000
    bus.write(0x0020, 0x00); // Low Byte
    bus.write(0x0021, 0x10); // High Byte

    // Put value 0x88 at $1005 ($1000 + $05)
    bus.write(0x1005, 0x88);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.y_register = 0x05;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x88);
    assert_eq!(cpu.cycles, 5, "Indirect Y (No Cross) should take 5 cycles");
}

#[test]
fn test_lda_indirect_y_page_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA ($20), Y
    bus.write(0x8000, 0xB1);
    bus.write(0x8001, 0x20);
    bus.write(0x0020, 0xFF); // Low Byte
    bus.write(0x0021, 0x10); // High Byte

    // Put value 0x77 at $1100 ($10FF + $01)
    bus.write(0x1100, 0x77);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.y_register = 0x01;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x77);
    assert_eq!(cpu.cycles, 6, "Indirect Y (Cross) should take 6 cycles");
}

#[test]
fn test_ldx_immediate_and_flags() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDX #$00 (Test Zero Flag)
    bus.write(0x8000, 0xA2);
    bus.write(0x8001, 0x00);

    // LDX #$FF (Test Negative Flag)
    bus.write(0x8002, 0xA2);
    bus.write(0x8003, 0xFF);

    cpu.registers.program_counter = 0x8000;

    // DX #$00
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.x_register, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    // LDX #$FF
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.x_register, 0xFF);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_ldy_immediate() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDY #$42
    bus.write(0x8000, 0xA0);
    bus.write(0x8001, 0x42);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.y_register, 0x42);
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_lda_indirect_x_zp_wrap_torture() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Set X to a value that forces the pointer to $FF
    // Base ($F0) + X ($0F) = $FF
    cpu.registers.x_register = 0x0F;

    // PROGRAM: LDA ($F0, X)
    // Code lives at $8000
    bus.write(0x8000, 0xA1);
    bus.write(0x8001, 0xF0);

    // POINTER SETUP (The Wrap)
    // We want to point to address $4000 (Safe location)
    // Low Byte ($00) goes to $00FF
    bus.write(0x00FF, 0x00);
    // High Byte ($40) goes to $0000 (Wrap!)
    bus.write(0x0000, 0x40);

    // If CPU does not wrap, it reads High Byte from $0100.
    bus.write(0x0100, 0x99);

    //   DATA
    // Put the value at $4000
    bus.write(0x4000, 0x42);

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x42, "Failed to wrap Zero Page pointer!");
}

#[test]
fn test_sta_absolute() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x8000, 0x8D);
    bus.write(0x8001, 0x00);
    bus.write(0x8002, 0x10);

    cpu.registers.accumulator = 0x55;
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x1000), 0x55);
    assert_eq!(cpu.cycles, 4, "STA should take 4 cycles");
}

#[test]
fn test_jmp_indirect_bug() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x30FF, 0x00);
    bus.write(0x3000, 0x80);

    // JMP ($30FF)
    bus.write(0x8000, 0x6C);
    bus.write(0x8001, 0xFF);
    bus.write(0x8002, 0x30);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.program_counter, 0x8000, "PC should jump to $8000");
}

#[test]
fn test_jmp() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    let program: Vec<u8> = vec![
        0xA9, 0x01, // 8000: LDA #$01 (Value)
        0x4C, 0x07, 0x80, // 8002: JMP $8007 (Target)
        0xA9, 0xFF, // 8005: LDA #$FF (We should not load this value!!)
        0x8D, 0x00, 0x00, // 8007: STA $0000 (Save Result)
    ];

    // load
    for (i, &byte) in program.iter().enumerate() {
        bus.write(0x8000 + i as u16, byte);
    }

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus); // LDA: 2 cycles
    cpu.step(&mut bus); // JMP: 3 cycles
    cpu.step(&mut bus); // STA: 4 cycles (if the JMP worked properly)

    assert_eq!(bus.read(0x0000), 0x01, "JMP failed!");
    assert_eq!(cpu.cycles, 9);
    assert_eq!(cpu.registers.program_counter, 0x800A, "PC ended up in the wrong place");
}

#[test]
fn test_txa_transfers_x_to_a() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0x10;
    cpu.registers.accumulator = 0x00;

    // TXA (0x8A)
    bus.write(0x8000, 0x8A);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x10);
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_txs_does_not_affect_flags() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0x00;
    cpu.registers.status.remove(Status::ZERO); // Clear Zero flag

    // TXS (0x9A) - Transfer X to Stack Pointer
    bus.write(0x8000, 0x9A);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.stack_pointer, 0x00);
    // TXS should NOT set Zero flag even if X is 0!
    assert_eq!(cpu.registers.status.contains(Status::ZERO), false);
}

#[test]
fn test_tsx_affects_flags() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.stack_pointer = 0x00;
    cpu.registers.x_register = 0xFF;

    // TSX (0xBA) - Transfer SP to X
    bus.write(0x8000, 0xBA);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0x00);
    // SX MUST set Zero flag if SP was 0!
    assert!(cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_register_transfers() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    let test_program: Vec<u8> = vec![
        0xA9, 0x42, // LDA #$42
        0xAA, // TAX (X = 42)
        0xA8, // TAY (Y = 42)
        0x8E, 0x01, 0x00, // STX $0001
        0x8C, 0x02, 0x00, // STY $0002
    ];

    // load
    for (i, &byte) in test_program.iter().enumerate() {
        bus.write(0x8000 + i as u16, byte);
    }
    cpu.registers.program_counter = 0x8000;

    // run
    for _ in 0..5 {
        cpu.step(&mut bus);
    }

    assert_eq!(cpu.registers.x_register, 0x42);
    assert_eq!(cpu.registers.y_register, 0x42);
    assert_eq!(bus.read(0x0001), 0x42, "STX failed");
    assert_eq!(bus.read(0x0002), 0x42, "STY failed");
}

#[test]
fn test_php_pushes_break_and_unused_flags() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.status = Status::empty();
    cpu.registers.stack_pointer = 0xFD;

    bus.write(0x8000, 0x08);
    bus.write(0x8000, 0x08);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    let pushed_value = bus.read(0x01FD);

    assert_eq!(pushed_value, 0x30, "PHP must push bits 4 and 5 as 1!");
    assert_eq!(cpu.registers.status.is_empty(), true, "PHP should not modify the CPU register");
    assert_eq!(cpu.registers.stack_pointer, 0xFC, "Stack pointer should decrement");
}
#[test]
fn test_plp_ignores_break_flag() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x01FD, 0xF0);
    cpu.registers.stack_pointer = 0xFC;

    bus.write(0x8000, 0x28);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert!(cpu.registers.status.contains(Status::NEGATIVE));
    assert!(cpu.registers.status.contains(Status::OVERFLOW));

    assert_eq!(
        cpu.registers.status.contains(Status::BRK),
        false,
        "PLP must ignore the Break flag from stack!"
    );

    assert!(cpu.registers.status.contains(Status::UNUSED), "Unused flag should always be 1");
    assert_eq!(cpu.registers.stack_pointer, 0xFD, "Stack pointer should increment");
}

#[test]
fn test_pha_pla_roundtrip() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // 1. PHA: Push 0x55
    bus.write(0x8000, 0x48);
    // 2. LDA: Load 0x00 (Corrupt A)
    bus.write(0x8001, 0xA9);
    bus.write(0x8002, 0x00);
    // 3. PLA: Restore A
    bus.write(0x8003, 0x68);

    cpu.registers.accumulator = 0x55;
    cpu.registers.program_counter = 0x8000;

    // Execute PHA
    cpu.step(&mut bus);
    assert_eq!(bus.read(0x01FD), 0x55, "Value 0x55 should be on stack at 0x01FD");
    assert_eq!(cpu.registers.stack_pointer, 0xFC, "SP should decrement");

    // Execute LDA #00
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.accumulator, 0x00);

    // Execute PLA
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.accumulator, 0x55, "PLA failed to restore 0x55");
    assert_eq!(cpu.registers.stack_pointer, 0xFD, "SP should increment");
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_jsr_rts_flow() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // PROGRAM START AT $8000
    // 1. JSR to $8004 (Opcode $20)
    //    Bytes: 20 04 80
    //    Instruction ends at $8002.
    //    JSR pushes $8002 (LAST byte of instruction) to stack.
    bus.write(0x8000, 0x20);
    bus.write(0x8001, 0x04);
    bus.write(0x8002, 0x80);

    // 2. Padding/Next Instruction at $8003 (Return target)
    //    We put a NOP here just to have something valid.
    bus.write(0x8003, 0xEA); // NOP

    // 3. SUBROUTINE at $8004
    //    RTS (Opcode $60)
    bus.write(0x8004, 0x60);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.stack_pointer = 0xFD;

    // --- STEP 1: EXECUTE JSR ---
    cpu.step(&mut bus);

    // Verify Jump
    assert_eq!(cpu.registers.program_counter, 0x8004, "PC should be at Subroutine ($8004)");

    // Verify Stack (Should contain $8002, Little Endian)
    // Stack Pointer starts at FD.
    // Push High (80) -> Goes to FD. SP -> FC.
    // Push Low  (02) -> Goes to FC. SP -> FB.
    assert_eq!(bus.read(0x01FD), 0x80, "Stack High Byte mismatch");
    assert_eq!(bus.read(0x01FC), 0x02, "Stack Low Byte mismatch");
    assert_eq!(cpu.registers.stack_pointer, 0xFB);

    // --- STEP 2: EXECUTE RTS ---
    cpu.step(&mut bus);

    // Verify Return
    // RTS pops $8002, adds 1 -> $8003.
    assert_eq!(cpu.registers.program_counter, 0x8003, "RTS should return to instruction AFTER JSR");
    assert_eq!(cpu.registers.stack_pointer, 0xFD, "Stack Pointer should return to original");
}

#[test]
fn test_nop() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // NOP (0xEA)
    bus.write(0x8000, 0xEA);

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.program_counter, 0x8001);
    assert_eq!(cpu.cycles, 2, "NOP should take 2 cycles");
}

#[test]
fn test_bne_logic() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // $8000: BNE +5 ($05) -> Jumps to $8007
    // $8007: LDA #$01 (Success)
    bus.write(0x8000, 0xD0);
    bus.write(0x8001, 0x05);
    bus.write(0x8007, 0xA9);
    bus.write(0x8008, 0x01);

    cpu.registers.program_counter = 0x8000;

    // Case 1: Zero Flag IS set (Should NOT branch)
    cpu.registers.status.insert(Status::ZERO);
    cpu.step(&mut bus);

    // PC should have advanced 2 bytes (8000 -> 8002)
    assert_eq!(cpu.registers.program_counter, 0x8002, "Should not branch if Z is set");

    // Zero Flag IS CLEAR (Should branch)
    // Reset PC to test again
    cpu.registers.program_counter = 0x8000;
    cpu.registers.status.remove(Status::ZERO);

    cpu.step(&mut bus); // Execute BNE

    // PC should be $8007
    // Math: $8000 (Opcode) + 1 (Fetch) = $8001. Fetch Offset ($8002).
    // Base PC for math is $8002.
    // $8002 + 5 = $8007.
    assert_eq!(cpu.registers.program_counter, 0x8007, "Should branch if Z is clear");
}

#[test]
fn test_beq_backward_jump() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Jump BACKWARDS
    // $8005: BEQ -3 ($FD) -> Target $8004
    bus.write(0x8005, 0xF0);
    bus.write(0x8006, 0xFD); // -3 in signed 8-bit

    cpu.registers.program_counter = 0x8005;
    cpu.registers.status.insert(Status::ZERO); // Force branch

    cpu.step(&mut bus);

    // Opcode read at $8005. PC -> $8006.
    // Offset read at $8006. PC -> $8007.
    // Base PC: $8007.
    // Offset: -3.
    // Target: $8007 - 3 = $8004.
    assert_eq!(cpu.registers.program_counter, 0x8004, "Backward jump failed");
}

#[test]
fn test_adc_overflow_flag() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // 1. Setup: 127 + 1 = 128 (-128 in i8)
    // Positive + Positive = Negative Result -> OVERFLOW!
    cpu.registers.accumulator = 0x7F; // 127
    bus.write(0x8000, 0x69); // ADC Immediate
    bus.write(0x8001, 0x01); // 1

    cpu.registers.program_counter = 0x8000;
    cpu.registers.status.remove(Status::CARRY); // Ensure Carry is clear

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x80); // Result is -128 (0x80)
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
    // CRITICAL: Overflow must be set because +127 + 1 is invalid in signed math
    assert!(cpu.registers.status.contains(Status::OVERFLOW));
    assert!(!cpu.registers.status.contains(Status::CARRY)); // No unsigned wrap (128 < 255)
}

#[test]
fn test_cmp_all_cases() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Program:
    // CMP #$10 (Where A = $10) -> Equal
    // CMP #$20 (Where A = $10) -> Less
    // CMP #$05 (Where A = $10) -> Greater

    bus.write(0x8000, 0xC9);
    bus.write(0x8001, 0x10); // CMP #$10
    bus.write(0x8002, 0xC9);
    bus.write(0x8003, 0x20); // CMP #$20
    bus.write(0x8004, 0xC9);
    bus.write(0x8005, 0x05); // CMP #$05

    cpu.registers.program_counter = 0x8000;
    cpu.registers.accumulator = 0x10; // Fixed A = 16

    // EQUAL (10 == 10)
    cpu.step(&mut bus);
    // Z=1 (Equal), C=1 (Because 10 >= 10)
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::CARRY));

    // LESS THAN (10 < 20)
    cpu.step(&mut bus);
    // Z=0 (Not Equal), C=0 (Borrow needed / Less Than), N=1 (10 - 20 = -10/F6)
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    // GREATER THAN (10 > 5)
    cpu.step(&mut bus);
    // Z=0 (Not Equal), C=1 (No Borrow / Greater)
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::CARRY));
}

#[test]
fn test_eor_logic() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // A = 0xFF (1111 1111)
    // M = 0xAA (1010 1010) (Immediate value)
    // Result Should be 0x55 (0101 0101)
    cpu.registers.accumulator = 0xFF;

    // Program: EOR #$AA
    bus.write(0x8000, 0x49);
    bus.write(0x8001, 0xAA);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x55, "EOR logic failed");

    // Flags: Result is Positive (Bit 7 is 0) and Not Zero.
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_bcc_bcs_logic() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // 1. BCC +5 (Branch if Carry Clear)
    bus.write(0x8000, 0x90);
    bus.write(0x8001, 0x05);

    // 2. BCS +10 (Branch if Carry Set)
    bus.write(0x8002, 0xB0);
    bus.write(0x8003, 0x0A);

    cpu.registers.program_counter = 0x8000;

    // Case 1: Carry is Set (Should NOT branch on BCC)
    cpu.registers.status.insert(Status::CARRY);
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.program_counter, 0x8002); // Fall through

    // Case 2: Carry is Set (Should branch on BCS)
    // PC is now at 8002. BCS + 0x0A. Target: 8004 + A = 800E.
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.program_counter, 0x800E);
}

#[test]
fn test_brk_rti_cycle() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // 1. Setup Vector
    bus.write(0xFFFE, 0x00);
    bus.write(0xFFFF, 0x90);

    // 2. Program: BRK then return target
    bus.write(0x8000, 0x00);
    bus.write(0x8001, 0xEA);
    bus.write(0x8002, 0xA9);
    bus.write(0x8003, 0x01);

    // 3. Handler: RTI
    bus.write(0x9000, 0x40);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.stack_pointer = 0xFD;
    cpu.cycles = 0; // Reset cycles

    // --- Execute BRK ---
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.program_counter, 0x9000);
    assert_eq!(cpu.cycles, 7, "BRK should take exactly 7 cycles");

    // Reset cycles for clarity
    cpu.cycles = 0;

    // --- Execute RTI ---
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.program_counter, 0x8002);
    assert_eq!(cpu.cycles, 6, "RTI should take exactly 6 cycles");
}
