use super::*;

#[test]
fn test_lda_immediate_timing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Program: LDA $10 (Immediate)
    bus.write(0x8000, 0xA9); // Opcode (Cycle 1)
    bus.write(0x8001, 0x10); // Operand: $10 (The value 16)

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.cycles, 2, "Immediate should take exactly 2 cycles");
    assert_eq!(cpu.registers.program_counter, 0x8002);
    assert_eq!(cpu.registers.accumulator, 0x10);
}

#[test]
fn test_lda_zeropage_timing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Program: LDA $44 (Zero Page)
    bus.write(0x8000, 0xA5); // Opcode (Cycle 1)
    bus.write(0x8001, 0x44); // Address $44 (Cycle 2)
    bus.write(0x0044, 0x55); // Data at target (Cycle 3)

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.cycles, 3, "Zero Page should take exactly 3 cycles");
    assert_eq!(cpu.registers.program_counter, 0x8002);
    assert_eq!(cpu.registers.accumulator, 0x55);
}

#[test]
fn test_lda_zeropage_x_timing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Program: LDA $44 (Zero Page X)
    bus.write(0x8000, 0xB5);
    bus.write(0x8001, 0x44);
    bus.write(0x0049, 0xAF);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x05;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.cycles, 4, "Zero Page should take exactly 4 cycles");
    assert_eq!(cpu.registers.program_counter, 0x8002);
    assert_eq!(cpu.registers.accumulator, 0xAF);
}

#[test]
fn test_lda_absolute() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA $1234
    bus.write(0x8000, 0xAD); // Opcode
    bus.write(0x8001, 0x34); // Low Byte
    bus.write(0x8002, 0x12); // High Byte
    bus.write(0x1234, 0x55);

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.cycles, 4, "Absolute should take exactly 4 cycles");
    assert_eq!(cpu.registers.program_counter, 0x8003);
    assert_eq!(cpu.registers.accumulator, 0x55);
}

#[test]
fn test_lda_absolute_x_no_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA $1000, X
    bus.write(0x8000, 0xBD);
    bus.write(0x8001, 0x00); // Low
    bus.write(0x8002, 0x10); // High
    bus.write(0x1005, 0xCD);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x05;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0xCD);
    assert_eq!(cpu.cycles, 4, "Absolute,X (No Cross) should take 4 cycles");
}

#[test]
fn test_lda_absolute_x_page_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA $10FF, X
    bus.write(0x8000, 0xBD);
    bus.write(0x8001, 0xFF);
    bus.write(0x8002, 0x10);
    bus.write(0x1100, 0x77);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x01;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x77);
    assert_eq!(cpu.cycles, 5, "Absolute,X (Cross) should take 5 cycles");
}

// Base: $2080, Y: $80 -> Target: $2100 ($2080 + $80 = $2100)
#[test]
fn test_lda_absolute_y_page_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x8000, 0xB9); // LDA Absolute,Y
    bus.write(0x8001, 0x80);
    bus.write(0x8002, 0x20);

    bus.write(0x2100, 0x88);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.y_register = 0x80;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x88);
    assert_eq!(cpu.cycles, 5, "Absolute,Y (Cross) should take 5 cycles");
}

#[test]
fn test_lda_indirect_x() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA ($20, X)
    bus.write(0x8000, 0xA1);
    bus.write(0x8001, 0x20);
    bus.write(0x0024, 0x34);
    bus.write(0x0025, 0x12);

    // Put the value 0x99 at $1234
    bus.write(0x1234, 0x99);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x04;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x99);
    assert_eq!(cpu.cycles, 6, "Indirect X should take 6 cycles");
}

#[test]
fn test_lda_indirect_y_no_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // DA ($20), Y
    bus.write(0x8000, 0xB1);
    bus.write(0x8001, 0x20); // Base ZP address

    // At $0020, store the address $1000
    bus.write(0x0020, 0x00); // Low Byte
    bus.write(0x0021, 0x10); // High Byte

    // Put value 0x88 at $1005 ($1000 + $05)
    bus.write(0x1005, 0x88);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.y_register = 0x05;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x88);
    assert_eq!(cpu.cycles, 5, "Indirect Y (No Cross) should take 5 cycles");
}

#[test]
fn test_lda_indirect_y_page_crossing() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDA ($20), Y
    bus.write(0x8000, 0xB1);
    bus.write(0x8001, 0x20);
    bus.write(0x0020, 0xFF); // Low Byte
    bus.write(0x0021, 0x10); // High Byte

    // Put value 0x77 at $1100 ($10FF + $01)
    bus.write(0x1100, 0x77);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.y_register = 0x01;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.accumulator, 0x77);
    assert_eq!(cpu.cycles, 6, "Indirect Y (Cross) should take 6 cycles");
}

#[test]
fn test_ldx_immediate_and_flags() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDX #$00 (Test Zero Flag)
    bus.write(0x8000, 0xA2);
    bus.write(0x8001, 0x00);

    // LDX #$FF (Test Negative Flag)
    bus.write(0x8002, 0xA2);
    bus.write(0x8003, 0xFF);

    cpu.registers.program_counter = 0x8000;

    // DX #$00
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.x_register, 0x00);
    assert!(cpu.registers.status.contains(Status::ZERO));
    assert!(!cpu.registers.status.contains(Status::NEGATIVE));

    // LDX #$FF
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.x_register, 0xFF);
    assert!(!cpu.registers.status.contains(Status::ZERO));
    assert!(cpu.registers.status.contains(Status::NEGATIVE));
}

#[test]
fn test_ldy_immediate() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // LDY #$42
    bus.write(0x8000, 0xA0);
    bus.write(0x8001, 0x42);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(cpu.registers.y_register, 0x42);
    assert!(!cpu.registers.status.contains(Status::ZERO));
}

#[test]
fn test_lda_indirect_x_zp_wrap_torture() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Set X to a value that forces the pointer to $FF
    // Base ($F0) + X ($0F) = $FF
    cpu.registers.x_register = 0x0F;

    // LDA ($F0, X)
    // Code lives at $8000
    bus.write(0x8000, 0xA1);
    bus.write(0x8001, 0xF0);

    // POINTER SETUP (The Wrap)
    // We want to point to address $4000 (Safe location)
    // Low Byte ($00) goes to $00FF
    bus.write(0x00FF, 0x00);
    // High Byte ($40) goes to $0000 (Wrap!)
    bus.write(0x0000, 0x40);

    // If CPU does not wrap, it reads High Byte from $0100.
    bus.write(0x0100, 0x99);

    //   DATA
    // Put the value at $4000
    bus.write(0x4000, 0x42);

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(
        cpu.registers.accumulator, 0x42,
        "Failed to wrap Zero Page pointer!"
    );
}

#[test]
fn test_sta_absolute() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x8000, 0x8D);
    bus.write(0x8001, 0x00);
    bus.write(0x8002, 0x10);

    cpu.registers.accumulator = 0x55;
    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(bus.read(0x1000), 0x55);
    assert_eq!(cpu.cycles, 4, "STA should take 4 cycles");
}

#[test]
fn test_register_transfers() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    let test_program: Vec<u8> = vec![
        0xA9, 0x42, // LDA #$42
        0xAA, // TAX (X = 42)
        0xA8, // TAY (Y = 42)
        0x8E, 0x01, 0x00, // STX $0001
        0x8C, 0x02, 0x00, // STY $0002
    ];

    // load
    for (i, &byte) in test_program.iter().enumerate() {
        bus.write(0x8000 + i as u16, byte);
    }
    cpu.registers.program_counter = 0x8000;

    // run
    for _ in 0..5 {
        cpu.step(&mut bus);
    }

    assert_eq!(cpu.registers.x_register, 0x42);
    assert_eq!(cpu.registers.y_register, 0x42);
    assert_eq!(bus.read(0x0001), 0x42, "STX failed");
    assert_eq!(bus.read(0x0002), 0x42, "STY failed");
}
