use super::*;

#[test]
fn test_jmp_indirect_bug() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    bus.write(0x30FF, 0x00);
    bus.write(0x3000, 0x80);

    // JMP ($30FF)
    bus.write(0x8000, 0x6C);
    bus.write(0x8001, 0xFF);
    bus.write(0x8002, 0x30);

    cpu.registers.program_counter = 0x8000;
    cpu.step(&mut bus);

    assert_eq!(
        cpu.registers.program_counter, 0x8000,
        "PC should jump to $8000"
    );
}

#[test]
fn test_jmp() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    let program: Vec<u8> = vec![
        0xA9, 0x01, // 8000: LDA #$01 (Value)
        0x4C, 0x07, 0x80, // 8002: JMP $8007 (Target)
        0xA9, 0xFF, // 8005: LDA #$FF (We should not load this value!!)
        0x8D, 0x00, 0x00, // 8007: STA $0000 (Save Result)
    ];

    // load
    for (i, &byte) in program.iter().enumerate() {
        bus.write(0x8000 + i as u16, byte);
    }

    cpu.registers.program_counter = 0x8000;
    cpu.cycles = 0;

    cpu.step(&mut bus); // LDA: 2 cycles
    cpu.step(&mut bus); // JMP: 3 cycles
    cpu.step(&mut bus); // STA: 4 cycles (if the JMP worked properly)

    assert_eq!(bus.read(0x0000), 0x01, "JMP failed!");
    assert_eq!(cpu.cycles, 9);
    assert_eq!(
        cpu.registers.program_counter, 0x800A,
        "PC ended up in the wrong place"
    );
}

#[test]
fn test_jsr_rts_flow() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // PROGRAM START AT $8000
    // JSR to $8004 (Opcode $20)
    // Bytes: 20 04 80
    // Instruction ends at $8002.
    // JSR pushes $8002 (LAST byte of instruction) to stack.
    bus.write(0x8000, 0x20);
    bus.write(0x8001, 0x04);
    bus.write(0x8002, 0x80);

    // Padding/Next Instruction at $8003 (Return target)
    // We put a NOP here just to have something valid.
    bus.write(0x8003, 0xEA); // NOP

    // SUBROUTINE at $8004
    // RTS (Opcode $60)
    bus.write(0x8004, 0x60);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.stack_pointer = 0xFD;

    cpu.step(&mut bus);

    // Verify Jump
    assert_eq!(
        cpu.registers.program_counter, 0x8004,
        "PC should be at Subroutine ($8004)"
    );

    // Stack Pointer starts at FD.
    // Push High (80) -> Goes to FD. SP -> FC.
    // Push Low  (02) -> Goes to FC. SP -> FB.
    assert_eq!(bus.read(0x01FD), 0x80, "Stack High Byte mismatch");
    assert_eq!(bus.read(0x01FC), 0x02, "Stack Low Byte mismatch");
    assert_eq!(cpu.registers.stack_pointer, 0xFB);

    cpu.step(&mut bus);

    // RTS pops $8002, adds 1 -> $8003.
    assert_eq!(
        cpu.registers.program_counter, 0x8003,
        "RTS should return to instruction AFTER JSR"
    );
    assert_eq!(
        cpu.registers.stack_pointer, 0xFD,
        "Stack Pointer should return to original"
    );
}

#[test]
fn test_brk_rti_cycle() {
    let mut bus = Memory::new();
    let mut cpu = CPU::new();

    // Setup Vector
    bus.write(0xFFFE, 0x00);
    bus.write(0xFFFF, 0x90);

    // BRK then return target
    bus.write(0x8000, 0x00);
    bus.write(0x8001, 0xEA);
    bus.write(0x8002, 0xA9);
    bus.write(0x8003, 0x01);

    // Handler: RTI
    bus.write(0x9000, 0x40);

    cpu.registers.program_counter = 0x8000;
    cpu.registers.stack_pointer = 0xFD;
    cpu.cycles = 0; // Reset cycles

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.program_counter, 0x9000);
    assert_eq!(cpu.cycles, 7, "BRK should take exactly 7 cycles");

    // Reset cycles for clarity
    cpu.cycles = 0;

    cpu.step(&mut bus);

    assert_eq!(cpu.registers.program_counter, 0x8002);
    assert_eq!(cpu.cycles, 6, "RTI should take exactly 6 cycles");
}
