use super::*;

#[test]
fn test_and_basic_logic() {
    let (mut cpu, mut bus) = setup();

    // A = 0xFF (1111 1111)
    // M = 0xAA (1010 1010)
    // Result = 0xAA (1010 1010)
    cpu.registers.accumulator = 0xFF;

    // Program: AND #$AA
    bus.write(0x8000, 0x29); // AND Immediate
    bus.write(0x8001, 0xAA);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0xAA, "AND logic failed");
    assert!(cpu.registers.status.contains(Status::NEGATIVE)); // Bit 7 is 1
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_and_zero_result() {
    let (mut cpu, mut bus) = setup();

    // A = 0x0F (0000 1111)
    // M = 0xF0 (1111 0000)
    // Result = 0x00 (0000 0000)
    cpu.registers.accumulator = 0x0F;

    // Program: AND #$F0
    bus.write(0x8000, 0x29); // AND Immediate
    bus.write(0x8001, 0xF0);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x00, "AND zero logic failed");
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
    assert!(cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_ora_basic_logic() {
    let (mut cpu, mut bus) = setup();

    // A = 0xAA (1010 1010)
    // M = 0x55 (0101 0101)
    // Result = 0xFF (1111 1111)
    cpu.registers.accumulator = 0xAA;

    // Program: ORA #$55
    bus.write(0x8000, 0x09); // ORA Immediate
    bus.write(0x8001, 0x55);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0xFF, "ORA logic failed");
    assert!(cpu.registers.status.contains(Status::NEGATIVE)); // Bit 7 is 1
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_ora_zero_result() {
    let (mut cpu, mut bus) = setup();

    // A = 0x00
    // M = 0x00
    // Result = 0x00
    cpu.registers.accumulator = 0x00;

    // Program: ORA #$00
    bus.write(0x8000, 0x09); // ORA Immediate
    bus.write(0x8001, 0x00);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x00, "ORA zero failed");
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
    assert!(cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_eor_basic_logic() {
    let (mut cpu, mut bus) = setup();

    // A = 0xFF (1111 1111)
    // M = 0xAA (1010 1010)
    // Result = 0x55 (0101 0101)
    cpu.registers.accumulator = 0xFF;

    // Program: EOR #$AA
    bus.write(0x8000, 0x49); // EOR Immediate
    bus.write(0x8001, 0xAA);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x55, "EOR logic failed");
    assert!(!cpu.registers.status.contains(Status::NEGATIVE)); // Bit 7 is 0
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_eor_zero_result() {
    let (mut cpu, mut bus) = setup();

    // A = 0xAA (1010 1010)
    // M = 0xAA (1010 1010)
    // Result = 0x00 (XOR with itself = 0)
    cpu.registers.accumulator = 0xAA;

    // Program: EOR #$AA
    bus.write(0x8000, 0x49); // EOR Immediate
    bus.write(0x8001, 0xAA);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x00, "EOR zero failed");
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
    assert!(cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_bit_all_flags() {
    let (mut cpu, mut bus) = setup();

    // A = 0xFF (1111 1111)
    // M = 0xC0 (1100 0000) - Bits 7 and 6 set
    // Result of AND = 0xC0 (non-zero)
    // N should be set from M bit 7
    // V should be set from M bit 6
    // Z should be clear (result != 0)
    cpu.registers.accumulator = 0xFF;

    // Store value in zero page
    bus.write(0x0010, 0xC0);

    // Program: BIT $10
    bus.write(0x8000, 0x24); // BIT Zero Page
    bus.write(0x8001, 0x10);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert!(
        cpu.registers.status.contains(Status::NEGATIVE),
        "N flag should be set from M bit 7"
    );
    assert!(
        cpu.registers.status.contains(Status::OVERFLOW),
        "V flag should be set from M bit 6"
    );
    assert!(
        !cpu.registers.status.contains(Status::ZERO),
        "Z flag should be clear (A & M != 0)"
    );

    // Accumulator should not change
    assert_eq!(cpu.registers.accumulator, 0xFF);
}

#[test]
fn test_bit_zero_result() {
    let (mut cpu, mut bus) = setup();

    // A = 0x0F (0000 1111)
    // M = 0xF0 (1111 0000)
    // Result of AND = 0x00 (zero)
    // N should be set from M bit 7
    // V should be set from M bit 6
    // Z should be set (result == 0)
    cpu.registers.accumulator = 0x0F;

    // Store value in zero page
    bus.write(0x0020, 0xF0);

    // Program: BIT $20
    bus.write(0x8000, 0x24); // BIT Zero Page
    bus.write(0x8001, 0x20);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert!(
        cpu.registers.status.contains(Status::NEGATIVE),
        "N flag should be set from M bit 7"
    );
    assert!(
        cpu.registers.status.contains(Status::OVERFLOW),
        "V flag should be set from M bit 6"
    );
    assert!(
        cpu.registers.status.contains(Status::ZERO),
        "Z flag should be set (A & M == 0)"
    );

    // Accumulator should not change
    assert_eq!(cpu.registers.accumulator, 0x0F);
}

#[test]
fn test_bit_no_overflow() {
    let (mut cpu, mut bus) = setup();

    // A = 0xFF
    // M = 0x80 (1000 0000) - Only bit 7 set
    // N should be set from M bit 7
    // V should be clear (M bit 6 is 0)
    // Z should be clear (result != 0)
    cpu.registers.accumulator = 0xFF;

    // Store value in zero page
    bus.write(0x0030, 0x80);

    // Program: BIT $30
    bus.write(0x8000, 0x24); // BIT Zero Page
    bus.write(0x8001, 0x30);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert!(
        cpu.registers.status.contains(Status::NEGATIVE),
        "N flag should be set from M bit 7"
    );
    assert!(
        !cpu.registers.status.contains(Status::OVERFLOW),
        "V flag should be clear (M bit 6 is 0)"
    );
    assert!(
        !cpu.registers.status.contains(Status::ZERO),
        "Z flag should be clear"
    );
}

#[test]
fn test_bit_only_overflow() {
    let (mut cpu, mut bus) = setup();

    // A = 0xFF
    // M = 0x40 (0100 0000) - Only bit 6 set
    // N should be clear (M bit 7 is 0)
    // V should be set from M bit 6
    // Z should be clear (result != 0)
    cpu.registers.accumulator = 0xFF;

    // Store value in zero page
    bus.write(0x0040, 0x40);

    // Program: BIT $40
    bus.write(0x8000, 0x24); // BIT Zero Page
    bus.write(0x8001, 0x40);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert!(
        !cpu.registers.status.contains(Status::NEGATIVE),
        "N flag should be clear (M bit 7 is 0)"
    );
    assert!(
        cpu.registers.status.contains(Status::OVERFLOW),
        "V flag should be set from M bit 6"
    );
    assert!(
        !cpu.registers.status.contains(Status::ZERO),
        "Z flag should be clear"
    );
}
