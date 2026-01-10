use super::*;

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
