use super::*;

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
    assert!(
        cpu.registers.status.is_empty(),
        "PHP should not modify the CPU register"
    );
    assert_eq!(
        cpu.registers.stack_pointer, 0xFC,
        "Stack pointer should decrement"
    );
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

    assert!(
        !cpu.registers.status.contains(Status::BRK),
        "PLP must ignore the Break flag from stack!"
    );

    assert!(
        cpu.registers.status.contains(Status::UNUSED),
        "Unused flag should always be 1"
    );
    assert_eq!(
        cpu.registers.stack_pointer, 0xFD,
        "Stack pointer should increment"
    );
}

#[test]
fn test_pha_pla_roundtrip() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Push 0x55
    bus.write(0x8000, 0x48);
    // Load 0x00 (Corrupt A)
    bus.write(0x8001, 0xA9);
    bus.write(0x8002, 0x00);
    // Restore A
    bus.write(0x8003, 0x68);

    cpu.registers.accumulator = 0x55;
    cpu.registers.program_counter = 0x8000;

    // Execute PHA
    cpu.step(&mut bus);
    assert_eq!(
        bus.read(0x01FD),
        0x55,
        "Value 0x55 should be on stack at 0x01FD"
    );
    assert_eq!(cpu.registers.stack_pointer, 0xFC, "SP should decrement");

    // Execute LDA #00
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.accumulator, 0x00);

    // Execute PLA
    cpu.step(&mut bus);
    assert_eq!(
        cpu.registers.accumulator, 0x55,
        "PLA failed to restore 0x55"
    );
    assert_eq!(cpu.registers.stack_pointer, 0xFD, "SP should increment");
    assert!(!cpu.registers.status.contains(Status::ZERO));
}
