use super::*;

#[test]
fn test_cmp_all_cases() {
    let (mut cpu, mut bus) = setup();
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
fn test_cpx_all_cases() {
    let (mut cpu, mut bus) = setup();
    // Program:
    // CPX #$10 (Where X = $10) -> Equal
    // CPX #$20 (Where X = $10) -> Less
    // CPX #$05 (Where X = $10) -> Greater

    bus.write(0x8000, 0xE0); // CPX Immediate
    bus.write(0x8001, 0x10); // CPX #$10
    bus.write(0x8002, 0xE0);
    bus.write(0x8003, 0x20); // CPX #$20
    bus.write(0x8004, 0xE0);
    bus.write(0x8005, 0x05); // CPX #$05

    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x10; // Fixed X = 16

    // EQUAL (X:16 == 16)
    cpu.step(&mut bus);
    // Z=1 (Equal), C=1 (Because 16 >= 16)
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    // LESS THAN (X:16 < 32)
    cpu.step(&mut bus);
    // Z=0 (Not Equal), C=0 (Borrow needed / Less Than), N=1 (16 - 32 = -16/F0)
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    // GREATER THAN (X:16 > 5)
    cpu.step(&mut bus);
    // Z=0 (Not Equal), C=1 (No Borrow / Greater)
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_cpy_all_cases() {
    let (mut cpu, mut bus) = setup();
    // Program:
    // CPY #$10 (Where Y = $10) -> Equal
    // CPY #$20 (Where Y = $10) -> Less
    // CPY #$05 (Where Y = $10) -> Greater

    bus.write(0x8000, 0xC0); // CPY Immediate
    bus.write(0x8001, 0x10); // CPY #$10
    bus.write(0x8002, 0xC0);
    bus.write(0x8003, 0x20); // CPY #$20
    bus.write(0x8004, 0xC0);
    bus.write(0x8005, 0x05); // CPY #$05

    cpu.registers.program_counter = 0x8000;
    cpu.registers.y_register = 0x10; // Fixed Y = 16

    // EQUAL (Y:16 == 16)
    cpu.step(&mut bus);
    // Z=1 (Equal), C=1 (Because 16 >= 16)
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    // LESS THAN (Y:16 < 32)
    cpu.step(&mut bus);
    // Z=0 (Not Equal), C=0 (Borrow needed / Less Than), N=1 (16 - 32 = -16/F0)
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::CARRY));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    // GREATER THAN (Y:16 > 5)
    cpu.step(&mut bus);
    // Z=0 (Not Equal), C=1 (No Borrow / Greater)
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_cpx_zero_comparison() {
    let (mut cpu, mut bus) = setup();

    // Test X = 0, M = 0 (edge case)
    cpu.registers.x_register = 0x00;

    bus.write(0x8000, 0xE0); // CPX Immediate
    bus.write(0x8001, 0x00);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_cpy_zero_comparison() {
    let (mut cpu, mut bus) = setup();

    // Test Y = 0, M = 0 (edge case)
    cpu.registers.y_register = 0x00;

    bus.write(0x8000, 0xC0); // CPY Immediate
    bus.write(0x8001, 0x00);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::CARRY));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
}
