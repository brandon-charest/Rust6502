use super::*;

#[test]
fn test_txa_transfers_x_to_a() {
    let (mut cpu, mut bus) = setup();

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
    let (mut cpu, mut bus) = setup();

    cpu.registers.x_register = 0x00;
    cpu.registers.status.remove(Status::ZERO); // Clear Zero flag

    // TXS (0x9A) - Transfer X to Stack Pointer
    bus.write(0x8000, 0x9A);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.stack_pointer, 0x00);
    // TXS should NOT set Zero flag even if X is 0!
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_tsx_affects_flags() {
    let (mut cpu, mut bus) = setup();

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
fn test_tax_transfers_a_to_x() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x42;
    cpu.registers.x_register = 0x00;

    // TAX (0xAA)
    bus.write(0x8000, 0xAA);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0x42);
    assert_eq!(cpu.registers.accumulator, 0x42); // A should not change
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_tax_zero_flag() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x00;
    cpu.registers.x_register = 0xFF;

    // TAX (0xAA)
    bus.write(0x8000, 0xAA);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_tax_negative_flag() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x80; // Bit 7 set
    cpu.registers.x_register = 0x00;

    // TAX (0xAA)
    bus.write(0x8000, 0xAA);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0x80);
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_tay_transfers_a_to_y() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x33;
    cpu.registers.y_register = 0x00;

    // TAY (0xA8)
    bus.write(0x8000, 0xA8);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.y_register, 0x33);
    assert_eq!(cpu.registers.accumulator, 0x33); // A should not change
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_tay_zero_flag() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0x00;
    cpu.registers.y_register = 0xFF;

    // TAY (0xA8)
    bus.write(0x8000, 0xA8);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.y_register, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_tay_negative_flag() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.accumulator = 0xFF;
    cpu.registers.y_register = 0x00;

    // TAY (0xA8)
    bus.write(0x8000, 0xA8);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.y_register, 0xFF);
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_tya_transfers_y_to_a() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.y_register = 0x25;
    cpu.registers.accumulator = 0x00;

    // TYA (0x98)
    bus.write(0x8000, 0x98);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x25);
    assert_eq!(cpu.registers.y_register, 0x25); // Y should not change
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_tya_zero_flag() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.y_register = 0x00;
    cpu.registers.accumulator = 0xFF;

    // TYA (0x98)
    bus.write(0x8000, 0x98);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_tya_negative_flag() {
    let (mut cpu, mut bus) = setup();

    cpu.registers.y_register = 0x90;
    cpu.registers.accumulator = 0x00;

    // TYA (0x98)
    bus.write(0x8000, 0x98);
    cpu.registers.program_counter = 0x8000;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x90);
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
    assert!(!cpu.registers.status.contains(Status::ZERO));
}
