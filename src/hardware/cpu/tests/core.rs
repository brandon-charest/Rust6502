use super::*;

#[test]
fn test_cpu_new() {
    let cpu = CPU::new();
    assert_eq!(cpu.registers.accumulator, 0);
    assert_eq!(cpu.registers.x_register, 0);
    assert_eq!(cpu.registers.y_register, 0);
    assert_eq!(cpu.registers.stack_pointer, 0xFD);
    assert_eq!(cpu.registers.program_counter, 0);
    assert_eq!(cpu.registers.status, Status::default());

    assert_eq!(cpu.cycles, 0);
    assert!(!cpu.halted);
}

#[test]
fn test_cpu_reset() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0xFFFC, 0x00);
    bus.write(0xFFFD, 0x80);

    cpu.reset(&mut bus);

    assert_eq!(cpu.registers.program_counter, 0x8000);
    assert_eq!(cpu.registers.stack_pointer, 0xFD);
    assert!(cpu.registers.status.contains(Status::DISABLE_INTERRUPTS));
}

#[test]
fn test_fetch_byte() {
    let bus = &mut Memory::new();
    let mut cpu = CPU::new();

    // Put an opcode at 0x8000
    let pc_start = 0x8000;
    cpu.registers.program_counter = pc_start;
    bus.write(pc_start, 0xEA); // NOP instruction

    let opcode = cpu.fetch_byte(bus);

    assert_eq!(opcode, 0xEA);
    assert_eq!(cpu.registers.program_counter, pc_start + 1);
}

#[test]
fn test_fetch_u16() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x2000, 0x34);
    bus.write(0x2001, 0x12);

    let value = cpu.read_word(&mut bus, 0x2000);

    assert_eq!(value, 0x1234);
}

#[test]
fn test_fetch_u16_wrapping() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0xFFFF, 0xAA);
    bus.write(0x0000, 0xBB);

    let value = cpu.read_word(&mut bus, 0xFFFF);

    assert_eq!(value, 0xBBAA);
}

#[test]
fn test_nop() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // NOP (0xEA)
    bus.write(0x8000, 0xEA);

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.program_counter, 0x8001);
    assert_eq!(cpu.cycles, 2, "NOP should take 2 cycles");
}
