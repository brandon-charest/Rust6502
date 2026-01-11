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
