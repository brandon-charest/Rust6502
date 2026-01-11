use super::*;

// ============================================================================
// ASL (Arithmetic Shift Left) Tests
// ============================================================================

#[test]
fn test_asl_accumulator_basic() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x40; // 0100_0000

    // ASL Accumulator (0x0A)
    bus.write(0x8000, 0x0A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x80); // 1000_0000
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
    assert!(!cpu.registers.status.contains(Status::ZERO));

    assert_eq!(
        cpu.cycles, 2,
        "test_asl_accumulator_basic should take 2 cycles"
    );
}

#[test]
fn test_asl_accumulator_sets_carry() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x81; // 1000_0001

    // ASL Accumulator (0x0A)
    bus.write(0x8000, 0x0A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x02); // 0000_0010
    assert!(cpu.registers.status.contains(Status::CARRY)); // Bit 7 was set
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
    assert!(!cpu.registers.status.contains(Status::ZERO));

    assert_eq!(
        cpu.cycles, 2,
        "test_asl_accumulator_sets_carry should take 2 cycles"
    );
}

#[test]
fn test_asl_accumulator_to_zero() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x80; // 1000_0000

    // ASL Accumulator (0x0A)
    bus.write(0x8000, 0x0A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x00);
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_asl_accumulator_to_zero should take 2 cycles"
    );
}

#[test]
fn test_asl_zero_page() {
    let (mut cpu, mut bus) = setup();

    bus.write(0x0042, 0x0F); // 0000_1111

    // ASL Zero Page (0x06)
    bus.write(0x8000, 0x06);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0042), 0x1E); // 0001_1110
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 5, "test_asl_zero_page should take 5 cycles");
}

// ============================================================================
// LSR (Logical Shift Right) Tests
// ============================================================================

#[test]
fn test_lsr_accumulator_basic() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x02; // 0000_0010

    // LSR Accumulator (0x4A)
    bus.write(0x8000, 0x4A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x01); // 0000_0001
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
    assert!(!cpu.registers.status.contains(Status::ZERO));

    assert_eq!(
        cpu.cycles, 2,
        "test_lsr_accumulator_basic should take 2 cycles"
    );
}

#[test]
fn test_lsr_accumulator_sets_carry() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x03; // 0000_0011

    // LSR Accumulator (0x4A)
    bus.write(0x8000, 0x4A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x01); // 0000_0001
    assert!(cpu.registers.status.contains(Status::CARRY)); // Bit 0 was set
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_lsr_accumulator_sets_carry should take 2 cycles"
    );
}

#[test]
fn test_lsr_accumulator_to_zero() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x01; // 0000_0001

    // LSR Accumulator (0x4A)
    bus.write(0x8000, 0x4A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x00);
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_lsr_accumulator_to_zero should take 2 cycles"
    );
}

#[test]
fn test_lsr_clears_negative_flag() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xFF; // 1111_1111

    // LSR Accumulator (0x4A)
    bus.write(0x8000, 0x4A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x7F); // 0111_1111
    assert!(cpu.registers.status.contains(Status::CARRY));
    // Negative flag must be clear because bit 7 is always 0 after LSR
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_lsr_clears_negative_flag should take 2 cycles"
    );
}

#[test]
fn test_lsr_zero_page() {
    let (mut cpu, mut bus) = setup();

    bus.write(0x0042, 0x1E); // 0001_1110

    // LSR Zero Page (0x46)
    bus.write(0x8000, 0x46);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0042), 0x0F); // 0000_1111
    assert!(!cpu.registers.status.contains(Status::CARRY));

    assert_eq!(cpu.cycles, 5, "test_lsr_zero_page should take 5 cycles");
}

// ============================================================================
// ROL (Rotate Left) Tests
// ============================================================================

#[test]
fn test_rol_accumulator_basic() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x40; // 0100_0000
    cpu.registers.status.remove(Status::CARRY);

    // ROL Accumulator (0x2A)
    bus.write(0x8000, 0x2A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x80); // 1000_0000 (old carry was 0)
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_rol_accumulator_basic should take 2 cycles"
    );
}

#[test]
fn test_rol_accumulator_with_carry_in() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x40; // 0100_0000
    cpu.registers.status.insert(Status::CARRY);

    // ROL Accumulator (0x2A)
    bus.write(0x8000, 0x2A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x81); // 1000_0001 (old carry was 1)
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_rol_accumulator_with_carry_in should take 2 cycles"
    );
}

#[test]
fn test_rol_accumulator_sets_carry() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x81; // 1000_0001
    cpu.registers.status.remove(Status::CARRY);

    // ROL Accumulator (0x2A)
    bus.write(0x8000, 0x2A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x02); // 0000_0010
    assert!(cpu.registers.status.contains(Status::CARRY)); // Bit 7 was set
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_rol_accumulator_sets_carry should take 2 cycles"
    );
}

#[test]
fn test_rol_rotates_through_carry() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xFF; // 1111_1111
    cpu.registers.status.insert(Status::CARRY);

    // ROL Accumulator (0x2A)
    bus.write(0x8000, 0x2A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0xFF); // 1111_1111 (rotated with carry in)
    assert!(cpu.registers.status.contains(Status::CARRY)); // Bit 7 was set
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_rol_rotates_through_carry should take 2 cycles"
    );
}

#[test]
fn test_rol_zero_page() {
    let (mut cpu, mut bus) = setup();

    bus.write(0x0042, 0x0F); // 0000_1111
    cpu.registers.status.remove(Status::CARRY);

    // ROL Zero Page (0x26)
    bus.write(0x8000, 0x26);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0042), 0x1E); // 0001_1110
    assert!(!cpu.registers.status.contains(Status::CARRY));

    assert_eq!(cpu.cycles, 5, "test_rol_zero_page should take 5 cycles");
}

// ============================================================================
// ROR (Rotate Right) Tests
// ============================================================================

#[test]
fn test_ror_accumulator_basic() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x02; // 0000_0010
    cpu.registers.status.remove(Status::CARRY);

    // ROR Accumulator (0x6A)
    bus.write(0x8000, 0x6A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x01); // 0000_0001 (old carry was 0)
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_ror_accumulator_basic should take 2 cycles"
    );
}

#[test]
fn test_ror_accumulator_with_carry_in() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x02; // 0000_0010
    cpu.registers.status.insert(Status::CARRY);

    // ROR Accumulator (0x6A)
    bus.write(0x8000, 0x6A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x81); // 1000_0001 (old carry was 1)
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE)); // Bit 7 set from carry

    assert_eq!(
        cpu.cycles, 2,
        "test_ror_accumulator_with_carry_in should take 2 cycles"
    );
}

#[test]
fn test_ror_accumulator_sets_carry() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x03; // 0000_0011
    cpu.registers.status.remove(Status::CARRY);

    // ROR Accumulator (0x6A)
    bus.write(0x8000, 0x6A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x01); // 0000_0001
    assert!(cpu.registers.status.contains(Status::CARRY)); // Bit 0 was set
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_ror_accumulator_sets_carry should take 2 cycles"
    );
}

#[test]
fn test_ror_rotates_through_carry() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xFF; // 1111_1111
    cpu.registers.status.insert(Status::CARRY);

    // ROR Accumulator (0x6A)
    bus.write(0x8000, 0x6A);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0xFF); // 1111_1111 (rotated with carry in)
    assert!(cpu.registers.status.contains(Status::CARRY)); // Bit 0 was set
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(
        cpu.cycles, 2,
        "test_ror_rotates_through_carry should take 2 cycles"
    );
}

#[test]
fn test_ror_zero_page() {
    let (mut cpu, mut bus) = setup();

    bus.write(0x0042, 0x1E); // 0001_1110
    cpu.registers.status.remove(Status::CARRY);

    // ROR Zero Page (0x66)
    bus.write(0x8000, 0x66);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0042), 0x0F); // 0000_1111
    assert!(!cpu.registers.status.contains(Status::CARRY));

    assert_eq!(cpu.cycles, 5, "test_ror_zero_page should take 5 cycles");
}

// ============================================================================
// Additional Memory Addressing Mode Tests
// ============================================================================

#[test]
fn test_asl_absolute() {
    let (mut cpu, mut bus) = setup();

    bus.write(0x1234, 0x55); // 0101_0101

    // ASL Absolute (0x0E)
    bus.write(0x8000, 0x0E);
    bus.write(0x8001, 0x34);
    bus.write(0x8002, 0x12);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x1234), 0xAA); // 1010_1010
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 6, "test_asl_absolute should take 6 cycles");
}

#[test]
fn test_lsr_absolute_x() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.x_register = 0x05;
    bus.write(0x1239, 0xAA); // 1010_1010

    // LSR Absolute,X (0x5E)
    bus.write(0x8000, 0x5E);
    bus.write(0x8001, 0x34);
    bus.write(0x8002, 0x12);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x1239), 0x55); // 0101_0101
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 7, "test_lsr_absolute_x should take 7 cycles");
}

#[test]
fn test_rol_absolute_x() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.x_register = 0x10;
    bus.write(0x1244, 0x7F); // 0111_1111
    cpu.registers.status.insert(Status::CARRY);

    // ROL Absolute,X (0x3E)
    bus.write(0x8000, 0x3E);
    bus.write(0x8001, 0x34);
    bus.write(0x8002, 0x12);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x1244), 0xFF); // 1111_1111 (with carry in)
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 7, "test_rol_absolute_x should take 7 cycles");
}

#[test]
fn test_ror_zero_page_x() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.x_register = 0x03;
    bus.write(0x0045, 0x01); // 0000_0001
    cpu.registers.status.insert(Status::CARRY);

    // ROR Zero Page,X (0x76)
    bus.write(0x8000, 0x76);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0045), 0x80); // 1000_0000 (carry rotated to bit 7)
    assert!(cpu.registers.status.contains(Status::CARRY)); // Bit 0 was set
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 6, "test_ror_zero_page_x should take 6 cycles");
}
