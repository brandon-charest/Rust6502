use super::*;

#[test]
fn test_inx_increments_x_register() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0x10;

    // INX (0xE8)
    bus.write(0x8000, 0xE8);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0x11);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));
    assert_eq!(cpu.cycles, 2, "INX should take 2 cycles");
}

#[test]
fn test_inx_wraps_around() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0xFF;

    // INX (0xE8)
    bus.write(0x8000, 0xE8);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 2, "INX should take 2 cycles");
}

#[test]
fn test_inx_sets_negative_flag() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0x7F;

    // INX (0xE8)
    bus.write(0x8000, 0xE8);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0x80);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 2, "INX should take 2 cycles");
}

#[test]
fn test_iny_increments_y_register() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.y_register = 0x10;

    // INY (0xC8)
    bus.write(0x8000, 0xC8);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.y_register, 0x11);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 2, "INY should take 2 cycles");
}

#[test]
fn test_iny_wraps_around() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.y_register = 0xFF;

    // INY (0xC8)
    bus.write(0x8000, 0xC8);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.y_register, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 2, "INY should take 2 cycles");
}

#[test]
fn test_dex_decrements_x_register() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0x10;

    // DEX (0xCA)
    bus.write(0x8000, 0xCA);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0x0F);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 2, "DEX should take 2 cycles");
}

#[test]
fn test_dex_wraps_around() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0x00;

    // DEX (0xCA)
    bus.write(0x8000, 0xCA);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0xFF);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 2, "DEX should take 2 cycles");
}

#[test]
fn test_dex_sets_zero_flag() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0x01;

    // DEX (0xCA)
    bus.write(0x8000, 0xCA);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.x_register, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 2, "DEX should take 2 cycles");
}

#[test]
fn test_dey_decrements_y_register() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.y_register = 0x10;

    // DEY (0x88)
    bus.write(0x8000, 0x88);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.y_register, 0x0F);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 2, "DEY should take 2 cycles");
}

#[test]
fn test_dey_wraps_around() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.y_register = 0x00;

    // DEY (0x88)
    bus.write(0x8000, 0x88);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.y_register, 0xFF);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 2, "DEY should take 2 cycles");
}

#[test]
fn test_inc_zero_page() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Set up value in zero page
    bus.write(0x0042, 0x10);

    // INC Zero Page (0xE6)
    bus.write(0x8000, 0xE6);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0042), 0x11);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 5, "INC Zero Page should take 5 cycles");
}

#[test]
fn test_inc_zero_page_x() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0x05;
    bus.write(0x0047, 0x20);

    // INC Zero Page,X (0xF6)
    bus.write(0x8000, 0xF6);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0047), 0x21);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 6, "INC Zero Page,X should take 6 cycles");
}

#[test]
fn test_inc_absolute() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x1234, 0x7F);

    // INC Absolute (0xEE)
    bus.write(0x8000, 0xEE);
    bus.write(0x8001, 0x34);
    bus.write(0x8002, 0x12);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x1234), 0x80);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 6, "INC Absolute should take 6 cycles");
}

#[test]
fn test_inc_wraps_to_zero() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x0042, 0xFF);

    // INC Zero Page (0xE6)
    bus.write(0x8000, 0xE6);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0042), 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 5, "INC Zero Page should take 5 cycles");
}

#[test]
fn test_dec_zero_page() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x0042, 0x10);

    // DEC Zero Page (0xC6)
    bus.write(0x8000, 0xC6);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0042), 0x0F);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 5, "DEC Zero Page should take 5 cycles");
}

#[test]
fn test_dec_zero_page_x() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    cpu.registers.x_register = 0x05;
    bus.write(0x0047, 0x01);

    // DEC Zero Page,X (0xD6)
    bus.write(0x8000, 0xD6);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0047), 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 6, "DEC Zero Page,X should take 6 cycles");
}

#[test]
fn test_dec_absolute() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x1234, 0x80);

    // DEC Absolute (0xCE)
    bus.write(0x8000, 0xCE);
    bus.write(0x8001, 0x34);
    bus.write(0x8002, 0x12);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x1234), 0x7F);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 6, "DEC Absolute should take 6 cycles");
}

#[test]
fn test_dec_wraps_to_ff() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x0042, 0x00);

    // DEC Zero Page (0xC6)
    bus.write(0x8000, 0xC6);
    bus.write(0x8001, 0x42);
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x0042), 0xFF);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));

    assert_eq!(cpu.cycles, 5, "DEC Zero Page should take 5 cycles");
}
