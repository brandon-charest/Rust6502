use super::*;

#[test]
fn test_adc_overflow_flag() {
    let (mut cpu, mut bus) = setup();

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
fn test_adc_basic_addition() {
    let (mut cpu, mut bus) = setup();

    // Basic addition: 10 + 5 = 15
    cpu.registers.accumulator = 0x0A; // 10
    cpu.registers.status.remove(Status::CARRY);

    bus.write(0x8000, 0x69); // ADC Immediate
    bus.write(0x8001, 0x05); // 5

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x0F); // 15
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::OVERFLOW));
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_adc_with_carry_in() {
    let (mut cpu, mut bus) = setup();

    // 10 + 5 + 1 (carry) = 16
    cpu.registers.accumulator = 0x0A; // 10
    cpu.registers.status.insert(Status::CARRY); // Carry set

    bus.write(0x8000, 0x69); // ADC Immediate
    bus.write(0x8001, 0x05); // 5

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x10); // 16
    assert!(!cpu.registers.status.contains(Status::CARRY));
}

#[test]
fn test_adc_carry_generation() {
    let (mut cpu, mut bus) = setup();

    // Unsigned overflow: 255 + 1 = 256 (wraps to 0, carry set)
    cpu.registers.accumulator = 0xFF;
    cpu.registers.status.remove(Status::CARRY);

    bus.write(0x8000, 0x69); // ADC Immediate
    bus.write(0x8001, 0x01);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x00);
    assert!(cpu.registers.status.contains(Status::CARRY)); // Carry set
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::OVERFLOW)); // -1 + 1 = 0 is valid
}

#[test]
fn test_adc_negative_overflow() {
    let (mut cpu, mut bus) = setup();

    // Negative + Negative = Positive overflow
    // -128 + -1 = -129 (invalid, wraps to +127)
    cpu.registers.accumulator = 0x80; // -128
    cpu.registers.status.remove(Status::CARRY);

    bus.write(0x8000, 0x69); // ADC Immediate
    bus.write(0x8001, 0xFF); // -1

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x7F); // 127
    assert!(cpu.registers.status.contains(Status::CARRY)); // Unsigned carry
    assert!(cpu.registers.status.contains(Status::OVERFLOW)); // Signed overflow
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_sbc_basic_subtraction() {
    let (mut cpu, mut bus) = setup();

    // 10 - 5 = 5 (with carry set, meaning no borrow)
    cpu.registers.accumulator = 0x0A; // 10
    cpu.registers.status.insert(Status::CARRY); // No borrow

    bus.write(0x8000, 0xE9); // SBC Immediate
    bus.write(0x8001, 0x05); // 5

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x05); // 5
    assert!(cpu.registers.status.contains(Status::CARRY)); // No borrow needed
    assert!(!cpu.registers.status.contains(Status::OVERFLOW));
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_sbc_with_borrow() {
    let (mut cpu, mut bus) = setup();

    // 10 - 5 - 1 (borrow) = 4
    cpu.registers.accumulator = 0x0A; // 10
    cpu.registers.status.remove(Status::CARRY); // Borrow needed

    bus.write(0x8000, 0xE9); // SBC Immediate
    bus.write(0x8001, 0x05); // 5

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x04); // 4
    assert!(cpu.registers.status.contains(Status::CARRY)); // No borrow for result
}

#[test]
fn test_sbc_zero_result() {
    let (mut cpu, mut bus) = setup();

    // 5 - 5 = 0
    cpu.registers.accumulator = 0x05;
    cpu.registers.status.insert(Status::CARRY); // No borrow

    bus.write(0x8000, 0xE9); // SBC Immediate
    bus.write(0x8001, 0x05);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::CARRY));
}

#[test]
fn test_sbc_underflow() {
    let (mut cpu, mut bus) = setup();

    // 0 - 1 = 255 (with carry set)
    cpu.registers.accumulator = 0x00;
    cpu.registers.status.insert(Status::CARRY);

    bus.write(0x8000, 0xE9); // SBC Immediate
    bus.write(0x8001, 0x01);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0xFF);
    assert!(!cpu.registers.status.contains(Status::CARRY)); // Borrow occurred
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_sbc_overflow_flag() {
    let (mut cpu, mut bus) = setup();

    // Signed overflow test: -128 - 1 = -129 (wraps to +127)
    // This is invalid in signed math
    cpu.registers.accumulator = 0x80; // -128
    cpu.registers.status.insert(Status::CARRY);

    bus.write(0x8000, 0xE9); // SBC Immediate
    bus.write(0x8001, 0x01); // 1

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x7F); // 127
    assert!(cpu.registers.status.contains(Status::OVERFLOW)); // Signed overflow
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_sbc_positive_overflow() {
    let (mut cpu, mut bus) = setup();

    // Positive - Negative = Positive overflow
    // 127 - (-1) = 128 (invalid positive result, wraps to -128)
    cpu.registers.accumulator = 0x7F; // 127
    cpu.registers.status.insert(Status::CARRY);

    bus.write(0x8000, 0xE9); // SBC Immediate
    bus.write(0x8001, 0xFF); // -1

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x80); // -128 (wrapped)
    assert!(!cpu.registers.status.contains(Status::CARRY)); // Borrow in unsigned
    assert!(cpu.registers.status.contains(Status::OVERFLOW)); // Signed overflow
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
}
