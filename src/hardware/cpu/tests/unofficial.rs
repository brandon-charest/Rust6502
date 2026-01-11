use super::*;

// ============================================================================
// LAX (Load Accumulator and X Register) Tests
// ============================================================================

#[test]
fn test_lax_zero_page() {
    let (mut cpu, mut bus) = setup();

    bus.write(0x0010, 0x7F);
    cpu.registers.accumulator = 0x00;
    cpu.registers.x_register = 0x00;

    // LAX Zero Page (0xA7)
    bus.write(0x8000, 0xA7);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x7F);
    assert_eq!(cpu.registers.x_register, 0x7F);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 3, "test_lax_zero_page should take 3 cycles");
}

#[test]
fn test_lax_sets_zero_flag() {
    let (mut cpu, mut bus) = setup();

    bus.write(0x0010, 0x00);
    cpu.registers.accumulator = 0xFF;
    cpu.registers.x_register = 0xFF;

    // LAX Zero Page (0xA7)
    bus.write(0x8000, 0xA7);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x00);
    assert_eq!(cpu.registers.x_register, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 3,
        "test_lax_sets_zero_flag should take 3 cycles"
    );
}

#[test]
fn test_lax_sets_negative_flag() {
    let (mut cpu, mut bus) = setup();

    bus.write(0x0010, 0x80);
    cpu.registers.accumulator = 0x00;
    cpu.registers.x_register = 0x00;

    // LAX Zero Page (0xA7)
    bus.write(0x8000, 0xA7);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x80);
    assert_eq!(cpu.registers.x_register, 0x80);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 3,
        "test_lax_sets_negative_flag should take 3 cycles"
    );
}

#[test]
fn test_lax_zero_page_y() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.y_register = 0x05;
    bus.write(0x0015, 0x33);
    cpu.registers.accumulator = 0x00;
    cpu.registers.x_register = 0x00;

    // LAX Zero Page,Y (0xB7)
    bus.write(0x8000, 0xB7);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x33);
    assert_eq!(cpu.registers.x_register, 0x33);

    assert_eq!(cpu.cycles, 4, "test_lax_zero_page_y should take 4 cycles");
}

#[test]
fn test_lax_absolute() {
    let (mut cpu, mut bus) = setup();

    bus.write(0x1234, 0x99);
    cpu.registers.accumulator = 0x00;
    cpu.registers.x_register = 0x00;

    // LAX Absolute (0xAF)
    bus.write(0x8000, 0xAF);
    bus.write(0x8001, 0x34);
    bus.write(0x8002, 0x12);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x99);
    assert_eq!(cpu.registers.x_register, 0x99);

    assert_eq!(cpu.cycles, 4, "test_lax_absolute should take 4 cycles");
}

// ============================================================================
// SAX (Store A AND X) Tests
// ============================================================================

#[test]
fn test_sax_zero_page() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xFF;
    cpu.registers.x_register = 0x0F;

    // SAX Zero Page (0x87)
    bus.write(0x8000, 0x87);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0010), 0x0F); // A & X = 0xFF & 0x0F = 0x0F

    assert_eq!(cpu.cycles, 3, "test_sax_zero_page should take 3 cycles");
}

#[test]
fn test_sax_zero_page_y() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xF0;
    cpu.registers.x_register = 0xCC;
    cpu.registers.y_register = 0x05;

    // SAX Zero Page,Y (0x97)
    bus.write(0x8000, 0x97);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0015), 0xC0); // A & X = 0xF0 & 0xCC = 0xC0

    assert_eq!(cpu.cycles, 4, "test_sax_zero_page_y should take 4 cycles");
}

#[test]
fn test_sax_absolute() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xAA;
    cpu.registers.x_register = 0x55;

    // SAX Absolute (0x8F)
    bus.write(0x8000, 0x8F);
    bus.write(0x8001, 0x34);
    bus.write(0x8002, 0x12);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x1234), 0x00); // A & X = 0xAA & 0x55 = 0x00

    assert_eq!(cpu.cycles, 4, "test_sax_absolute should take 4 cycles");
}

#[test]
fn test_sax_absolute_y() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x3C;
    cpu.registers.x_register = 0x69;
    cpu.registers.y_register = 0x10;

    // SAX Absolute,Y (0x83)
    bus.write(0x8000, 0x83);
    bus.write(0x8001, 0x00);
    bus.write(0x8002, 0x50);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x5010), 0x28); // A & X = 0x3C & 0x69 = 0x28

    assert_eq!(cpu.cycles, 5, "test_sax_absolute_y should take 5 cycles");
}

// ============================================================================
// DCP (Decrement and Compare) Tests
// ============================================================================

#[test]
fn test_dcp_zero_page_basic() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x50;
    bus.write(0x0010, 0x60);

    // DCP Zero Page (0xC7)
    bus.write(0x8000, 0xC7);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory should be decremented: 0x60 -> 0x5F
    assert_eq!(bus.read(0x0010), 0x5F);

    // Compare A (0x50) with decremented value (0x5F)
    // 0x50 < 0x5F, so Carry should be clear
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::NEGATIVE)); // (0x50 - 0x5F) is negative

    assert_eq!(
        cpu.cycles, 5,
        "test_dcp_zero_page_basic should take 5 cycles"
    );
}

#[test]
fn test_dcp_sets_zero_flag() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x5F;
    bus.write(0x0010, 0x60);

    // DCP Zero Page (0xC7)
    bus.write(0x8000, 0xC7);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory: 0x60 -> 0x5F
    assert_eq!(bus.read(0x0010), 0x5F);

    // Compare A (0x5F) with decremented value (0x5F)
    // They're equal, so Zero and Carry should be set
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 5,
        "test_dcp_sets_zero_flag should take 5 cycles"
    );
}

#[test]
fn test_dcp_sets_carry_flag() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x70;
    bus.write(0x0010, 0x60);

    // DCP Zero Page (0xC7)
    bus.write(0x8000, 0xC7);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory: 0x60 -> 0x5F
    assert_eq!(bus.read(0x0010), 0x5F);

    // Compare A (0x70) with decremented value (0x5F)
    // 0x70 >= 0x5F, so Carry should be set
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::ZERO));

    assert_eq!(
        cpu.cycles, 5,
        "test_dcp_sets_carry_flag should take 5 cycles"
    );
}

#[test]
fn test_dcp_wraps_around() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xFF;
    bus.write(0x0010, 0x00);

    // DCP Zero Page (0xC7)
    bus.write(0x8000, 0xC7);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory: 0x00 -> 0xFF (wraps around)
    assert_eq!(bus.read(0x0010), 0xFF);

    // Compare A (0xFF) with decremented value (0xFF)
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::ZERO));

    assert_eq!(cpu.cycles, 5, "test_dcp_wraps_around should take 5 cycles");
}

#[test]
fn test_dcp_absolute() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x80;
    bus.write(0x1234, 0x81);

    // DCP Absolute (0xCF)
    bus.write(0x8000, 0xCF);
    bus.write(0x8001, 0x34);
    bus.write(0x8002, 0x12);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory: 0x81 -> 0x80
    assert_eq!(bus.read(0x1234), 0x80);

    // Compare A (0x80) with decremented value (0x80)
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::ZERO));

    assert_eq!(cpu.cycles, 6, "test_dcp_absolute should take 6 cycles");
}

// ============================================================================
// RLA (Rotate Left and AND) Tests
// ============================================================================

#[test]
fn test_rla_zero_page_basic() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xFF;
    bus.write(0x0010, 0x40); // 0100_0000
    cpu.registers.status.remove(Status::CARRY);

    // RLA Zero Page (0x27)
    bus.write(0x8000, 0x27);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory rotated left: 0x40 (0100_0000) -> 0x80 (1000_0000)
    assert_eq!(bus.read(0x0010), 0x80);

    // Accumulator AND with rotated value: 0xFF & 0x80 = 0x80
    assert_eq!(cpu.registers.accumulator, 0x80);

    // Carry flag should be clear (bit 7 of 0x40 was 0)
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
    assert!(!cpu.registers.status.contains(Status::ZERO));

    assert_eq!(
        cpu.cycles, 5,
        "test_rla_zero_page_basic should take 5 cycles"
    );
}

#[test]
fn test_rla_with_carry_in() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xFF;
    bus.write(0x0010, 0x40); // 0100_0000
    cpu.registers.status.insert(Status::CARRY); // Set carry

    // RLA Zero Page (0x27)
    bus.write(0x8000, 0x27);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory rotated left with carry: 0x40 (0100_0000) -> 0x81 (1000_0001)
    assert_eq!(bus.read(0x0010), 0x81);

    // Accumulator AND with rotated value: 0xFF & 0x81 = 0x81
    assert_eq!(cpu.registers.accumulator, 0x81);

    // Carry flag should be clear (bit 7 of 0x40 was 0)
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 5, "test_rla_with_carry_in should take 5 cycles");
}

#[test]
fn test_rla_sets_carry() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xFF;
    bus.write(0x0010, 0x81); // 1000_0001
    cpu.registers.status.remove(Status::CARRY);

    // RLA Zero Page (0x27)
    bus.write(0x8000, 0x27);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory rotated left: 0x81 (1000_0001) -> 0x02 (0000_0010)
    assert_eq!(bus.read(0x0010), 0x02);

    // Accumulator AND with rotated value: 0xFF & 0x02 = 0x02
    assert_eq!(cpu.registers.accumulator, 0x02);

    // Carry flag should be set (bit 7 of 0x81 was 1)
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 5, "test_rla_sets_carry should take 5 cycles");
}

#[test]
fn test_rla_results_in_zero() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x00;
    bus.write(0x0010, 0x7F); // 0111_1111
    cpu.registers.status.remove(Status::CARRY);

    // RLA Zero Page (0x27)
    bus.write(0x8000, 0x27);
    bus.write(0x8001, 0x10);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory rotated left: 0x7F -> 0xFE
    assert_eq!(bus.read(0x0010), 0xFE);

    // Accumulator AND with rotated value: 0x00 & 0xFE = 0x00
    assert_eq!(cpu.registers.accumulator, 0x00);

    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 5,
        "test_rla_results_in_zero should take 5 cycles"
    );
}

#[test]
fn test_rla_absolute() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x0F;
    bus.write(0x1234, 0x55); // 0101_0101
    cpu.registers.status.insert(Status::CARRY);

    // RLA Absolute (0x2F)
    bus.write(0x8000, 0x2F);
    bus.write(0x8001, 0x34);
    bus.write(0x8002, 0x12);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // Memory rotated left with carry: 0x55 -> 0xAB (1010_1011)
    assert_eq!(bus.read(0x1234), 0xAB);

    // Accumulator AND with rotated value: 0x0F & 0xAB = 0x0B
    assert_eq!(cpu.registers.accumulator, 0x0B);

    assert_eq!(cpu.cycles, 6, "test_rla_absolute should take 6 cycles");
}

// ============================================================================
// KIL (Halt/Crash CPU) Tests
// ============================================================================

#[test]
fn test_kil_halts_cpu() {
    let (mut cpu, mut bus) = setup();

    assert!(!cpu.halted); // CPU should not be halted initially

    // KIL (0x02) - There are multiple KIL opcodes
    bus.write(0x8000, 0x02);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    // CPU should now be halted
    assert!(cpu.halted);

    assert_eq!(cpu.cycles, 1, "test_kil_halts_cpu should take 1 cycles");
}

#[test]
fn test_kil_prevents_execution() {
    let (mut cpu, mut bus) = setup();

    // KIL (0x02)
    bus.write(0x8000, 0x02);
    // LDA #$42 - This should never execute
    bus.write(0x8001, 0xA9);
    bus.write(0x8002, 0x42);

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;
    cpu.registers.accumulator = 0x00;

    // Execute KIL
    cpu.step(&mut bus);
    assert!(cpu.halted);

    let cycles_after_kil = cpu.cycles;

    // Try to execute next instruction - should not execute because CPU is halted
    cpu.step(&mut bus);

    // Accumulator should still be 0x00 (LDA didn't execute)
    assert_eq!(cpu.registers.accumulator, 0x00);

    // Only 1 cycle should have passed (halted state increments cycles by 1)
    assert_eq!(cpu.cycles, cycles_after_kil + 1);

    assert_eq!(
        cpu.cycles, 2,
        "test_kil_prevents_execution: 1 for KIL + 1 for halted step = 2 cycles total"
    );
}
