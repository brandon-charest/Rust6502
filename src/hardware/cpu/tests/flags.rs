use super::*;

#[test]
fn test_clc_clears_carry_flag() {
    let mut cpu = CPU::new();

    cpu.registers.status.insert(Status::CARRY);
    assert!(cpu.registers.status.contains(Status::CARRY));

    clc(&mut cpu);

    assert!(!cpu.registers.status.contains(Status::CARRY));
}

#[test]
fn test_sec_sets_carry_flag() {
    let mut cpu = CPU::new();

    cpu.registers.status.remove(Status::CARRY);
    assert!(!cpu.registers.status.contains(Status::CARRY));

    sec(&mut cpu);

    assert!(cpu.registers.status.contains(Status::CARRY));
}

#[test]
fn test_cli_clears_interrupt_disable() {
    let mut cpu = CPU::new();

    cpu.registers.status.insert(Status::DISABLE_INTERRUPTS);

    cli(&mut cpu);

    assert!(!cpu.registers.status.contains(Status::DISABLE_INTERRUPTS));
}

#[test]
fn test_sei_sets_interrupt_disable() {
    let mut cpu = CPU::new();

    cpu.registers.status.remove(Status::DISABLE_INTERRUPTS);

    sei(&mut cpu);

    assert!(cpu.registers.status.contains(Status::DISABLE_INTERRUPTS));
}

#[test]
fn test_clv_clears_overflow_flag() {
    let mut cpu = CPU::new();

    cpu.registers.status.insert(Status::OVERFLOW);

    clv(&mut cpu);

    assert!(!cpu.registers.status.contains(Status::OVERFLOW));
}

#[test]
fn test_cld_clears_decimal_flag() {
    let mut cpu = CPU::new();

    cpu.registers.status.insert(Status::DECIMAL_MODE);

    cld(&mut cpu);

    assert!(!cpu.registers.status.contains(Status::DECIMAL_MODE));
}

#[test]
fn test_sed_sets_decimal_flag() {
    let mut cpu = CPU::new();

    cpu.registers.status.remove(Status::DECIMAL_MODE);

    sed(&mut cpu);

    assert!(cpu.registers.status.contains(Status::DECIMAL_MODE));
}

#[test]
fn test_flag_ops_do_not_affect_other_flags() {
    let mut cpu = CPU::new();

    cpu.registers.status.insert(Status::NEGATIVE);
    cpu.registers.status.remove(Status::CARRY);

    sec(&mut cpu);

    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
}
