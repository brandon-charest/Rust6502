use super::*;

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
